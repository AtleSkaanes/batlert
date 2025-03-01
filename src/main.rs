mod cli;
mod popup;
use clap::Parser;
use popup::Choice;
use rodio::{Decoder, OutputStream, Sink};
use std::{
    fs::OpenOptions,
    io::{BufReader, Cursor, Read, Seek},
    path::Path,
    process::Command,
    time::Duration,
};

fn main() {
    let args = cli::CliArgs::parse();

    let s = format!("/sys/class/power_supply/{}/capacity", args.battery);
    let battery_path = Path::new(&s);
    let status_path = format!("/sys/class/power_supply/{}/status", args.battery);

    let mut battery_file = match OpenOptions::new().read(true).open(battery_path) {
        Ok(f) => f,
        Err(_) => {
            eprintln!("[ERROR]: Couldn't find battery {}", args.battery);
            std::process::exit(1);
        }
    };

    let mut status_file = match OpenOptions::new().read(true).open(status_path) {
        Ok(f) => f,
        Err(_) => {
            eprintln!("[ERROR]: Couldn't read battery status");
            std::process::exit(1);
        }
    };

    // If the popup has been called before on this discharge.
    // So the popup isn't called everytime the battery loses a pct
    let mut has_been_called = false;
    loop {
        std::thread::sleep(Duration::from_secs(args.interval));

        let mut cap_buf = String::new();
        match battery_file.read_to_string(&mut cap_buf) {
            Ok(_) => {}
            Err(e) => {
                eprintln!("[ERROR]: Failed to read battery capacity.\nDetails: {}", e);
            }
        };
        // Rewind the starting read position to the start of the file
        battery_file.rewind().unwrap();

        let Ok(pct) = cap_buf.trim().parse::<u32>() else {
            eprintln!("[ERROR]: {:?} does not hold a whole number", battery_path);
            continue;
        };

        let mut status_buf = String::new();
        match status_file.read_to_string(&mut status_buf) {
            Ok(_) => {}
            Err(e) => {
                eprintln!("[ERROR]: Failed to read battery status.\nDetails: {}", e);
            }
        };
        // Rewind the starting read position to the start of the file
        status_file.rewind().unwrap();

        if status_buf.trim() != "Discharging" {
            has_been_called = false;
            continue;
        }

        if pct <= args.battery_pct && !has_been_called {
            has_been_called = true;
            //let answer = popup::create_popup(&args);

            let mut answer = None;
            while answer.is_none() {
                play_sound(&args);
                answer = popup::create_popup(&args);
            }

            evaluate_choice(answer, &args);
        }
    }
}

fn evaluate_choice(answer: Option<Choice>, args: &cli::CliArgs) {
    let cmd = match answer.unwrap_or(Choice::Ok) {
        Choice::Suspend => args.suspend_cmd.clone(),
        Choice::Ok => "echo \"Ignoring popup\"".to_owned(),
        Choice::Shutdown => args.shutdown_cmd.clone(),
    };

    match Command::new("sh").arg("-c").arg(cmd.clone()).output() {
        Ok(out) => {
            println!(
                "command \"{}\" evaluates to:\n{}",
                cmd,
                String::from_utf8(out.stdout).unwrap_or_default()
            )
        }
        Err(e) => {
            eprintln!(
                "[ERROR]: Failed to run command \"{}\". Error details: {}",
                cmd.clone(),
                e
            );
        }
    }
}

fn play_sound(args: &cli::CliArgs) {
    let sound_path = args.sound.clone();
    std::thread::spawn(move || {
        let alert_sound_default = include_bytes!("../sounds/ping.oga");

        let (_stream, stream_handle) = OutputStream::try_default().unwrap();
        let sink = Sink::try_new(&stream_handle).unwrap();

        match sound_path {
            Some(path) => {
                // Don't play a sound if path is empty
                if path.trim().is_empty() {
                    return;
                }
                let Ok(file) = OpenOptions::new().read(true).open(path.clone()) else {
                    eprintln!("[ERROR]: Sound file at {} not found", &path);
                    return;
                };

                let buf = BufReader::new(file);
                let source = Decoder::new(buf).unwrap();
                sink.append(source);
            }
            None => {
                let buf = BufReader::new(Cursor::new(alert_sound_default));
                let source = Decoder::new(buf).unwrap();
                sink.append(source);
            }
        };
        sink.sleep_until_end();
    });
}
