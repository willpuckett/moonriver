pub mod app;
pub mod event;
pub mod modal;
pub mod printer;
pub mod stateful_list;
pub mod tabs;
pub mod ui;
pub mod widgets;

use std::io;
use std::time::Duration;

use crossterm::{
    event::{DisableMouseCapture, EnableMouseCapture},
    execute,
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
};
use ratatui::{backend::CrosstermBackend, Terminal};

pub use app::App;
pub use event::EventHandler;

pub type Result<T> = anyhow::Result<T>;

/// Initialize the TUI terminal
pub fn init() -> Result<Terminal<CrosstermBackend<io::Stdout>>> {
    enable_raw_mode()?;
    let mut stdout = io::stdout();
    execute!(stdout, EnterAlternateScreen, EnableMouseCapture)?;
    let backend = CrosstermBackend::new(stdout);
    let terminal = Terminal::new(backend)?;
    Ok(terminal)
}

/// Restore the terminal to its original state
pub fn restore() -> Result<()> {
    disable_raw_mode()?;
    execute!(io::stdout(), LeaveAlternateScreen, DisableMouseCapture)?;
    Ok(())
}

/// Main TUI event loop
pub async fn run(
    terminal: &mut Terminal<CrosstermBackend<io::Stdout>>,
    app: &mut App,
) -> Result<()> {
    let mut event_handler = EventHandler::new(Duration::from_millis(100));

    loop {
        // Draw the UI
        terminal.draw(|frame| ui::render(app, frame))?;

        // Handle events
        if let Some(event) = event_handler.next().await
            && !app.handle_event(event).await? {
                break;
            }

        // Update app state from WebSocket messages
        app.update().await?;
    }

    Ok(())
}
