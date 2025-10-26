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

    /// Launch REPL (Read-Eval-Print Loop) mode instead of TUI
    #[arg(long)]
    pub repl: bool,

    /// Execute commands in non-interactive mode (implies --repl)
    /// Multiple commands can be separated by commas
    #[arg(long, short = 'c')]
    pub command: Option<String>,
}
