use clap::Parser;

#[derive(Debug, Parser)]
#[command(version, about)]
pub struct CliArgs {
    /// The command that will run when the "shutdown" action is selected.
    /// Defaults to `shutdown`
    #[arg(long)]
    #[clap(default_value = "shutdown")]
    pub shutdown_cmd: String,
    /// The command that will run when the "suspend" action is selected.
    /// Defaults to `systemctl suspend`
    #[arg(long)]
    #[clap(default_value = "systemctl suspend")]
    pub suspend_cmd: String,
    /// The command that will be called when the popup is activated
    #[arg(long)]
    pub on_popup: Option<String>,
    /// The path to the sound that will play when the popup is activated.
    /// Leave this is as an empty string if you want to disable sound.
    #[arg(long)]
    pub sound: Option<String>,
    /// Specifies what battery percentage the popup will be activated at, as an integer between 0..100.
    /// Defaults to 5.
    #[arg(long)]
    #[clap(default_value = "5")]
    pub battery_pct: u32,
    /// The device name for the battery that should be used.
    /// Defaults to "BAT0"
    #[arg(long)]
    #[clap(default_value = "BAT0")]
    pub battery: String,
    /// The interval between battery checks in seconds.
    /// Defaults to 300 seconds (2.5 min)
    #[arg(long)]
    #[clap(default_value = "150")]
    pub interval: u64,
    /// Specifies arguments that will be passed to the GTK application
    #[arg(long)]
    pub gtk_args: Option<Vec<String>>,
}
