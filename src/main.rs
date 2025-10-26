mod cli;
mod config;
mod moonraker;
mod repl;
mod tui;

use anyhow::Result;
use clap::Parser;
use cli::Cli;

#[tokio::main]
async fn main() -> Result<()> {
    let cli = Cli::parse();

    // Load configuration
    let config = config::load_config()?;

    // Build the Moonraker URL
    let url = format!("ws://{}:{}/websocket", cli.host, cli.port);
    let server_url = format!("{}:{}", cli.host, cli.port);

    // If we have a command to execute (scripting mode)
    if let Some(command_str) = &cli.command {
        // Connect to Moonraker
        let mut client = moonraker::MoonrakerClient::connect(&url).await?;
        
        // Split by comma to support multiple commands
        let commands: Vec<&str> = command_str.split(',').map(|s| s.trim()).collect();

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

    // If REPL mode is explicitly requested
    if cli.repl {
        let mut client = moonraker::MoonrakerClient::connect(&url).await?;
        repl::run_repl(client).await?;
        return Ok(());
    }

    // Default: Launch TUI mode
    // Initialize terminal
    let mut terminal = tui::init()?;
    
    // Create app state
    let mut app = tui::App::new(server_url, config);

    // Connect to Moonraker in background
    match moonraker::MoonrakerClient::connect(&url).await {
        Ok(client) => {
            app.set_client(client);
        }
        Err(e) => {
            eprintln!("Warning: Failed to connect to Moonraker: {}", e);
            eprintln!("TUI will launch without printer connection");
        }
    }

    // Run the TUI
    let result = tui::run(&mut terminal, &mut app).await;

    // Restore terminal
    tui::restore()?;

    result
}
