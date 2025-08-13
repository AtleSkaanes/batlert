pub fn fatal(msg: &str) -> ! {
    eprintln!("\x1b[31m[FATAL ERR]: {}\x1b[0m", msg);
    std::process::exit(1);
}

macro_rules! fatalf {
    ($($arg:tt)*) => {
        crate::log::fatal(&::std::format!($($arg)*));
    };
}

pub(crate) use fatalf;

pub fn err(msg: &str) {
    eprintln!("\x1b[31m[ERR]: {}\x1b[0m", msg);
}

macro_rules! errf {
    ($($arg:tt)*) => {
        crate::log::err(&::std::format!($($arg)*))
    };
}

pub(crate) use errf;

pub fn status(msg: &str) {
    eprintln!("\x1b[34m[STATUS]: {}\x1b[0m", msg);
}

macro_rules! statusf {
    ($($arg:tt)*) => {
        crate::log::status(&::std::format!($($arg)*))
    };
}

pub(crate) use statusf;
