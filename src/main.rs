mod cli;
mod config;
mod moonraker;
mod repl;

use anyhow::Result;
use clap::Parser;
use cli::Cli;

#[tokio::main]
async fn main() -> Result<()> {
    let cli = Cli::parse();

    // Build the Moonraker URL
    let url = format!("ws://{}:{}/websocket", cli.host, cli.port);

    // Connect to Moonraker
    let mut client = moonraker::MoonrakerClient::connect(&url).await?;

    // If we have commands to execute (scripting mode), run them and exit
    if !cli.commands.is_empty() {
        let commands_str = cli.commands.join(" ");
        // Split by comma to support multiple commands
        let commands: Vec<&str> = commands_str.split(',').map(|s| s.trim()).collect();

        for cmd in commands {
            if !cmd.is_empty() {
                client.send_gcode(cmd).await?;
                // Give some time for response
                tokio::time::sleep(tokio::time::Duration::from_millis(500)).await;
            }
        }

        client.disconnect().await?;
        return Ok(());
    }

    // Otherwise, start interactive REPL
    repl::run_repl(client).await?;

    Ok(())
}
