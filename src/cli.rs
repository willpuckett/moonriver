use clap::Parser;

#[derive(Parser, Debug)]
#[command(
    name = "moonriver",
    version,
    about = "A terminal-based console for connecting to Klipper via Moonraker WebSocket API",
    long_about = "Moonriver provides a fast, efficient, and color-coded way to monitor and control your 3D printer from the command line."
)]
pub struct Cli {
    /// Moonraker host address
    #[arg(long, default_value = "localhost")]
    pub host: String,

    /// Moonraker port
    #[arg(long, default_value = "7125")]
    pub port: u16,

    /// API key for authentication (if required)
    #[arg(long)]
    pub api_key: Option<String>,

    /// Commands to execute in non-interactive mode (separated by commas)
    #[arg(trailing_var_arg = true)]
    pub commands: Vec<String>,
}
