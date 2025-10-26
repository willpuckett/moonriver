use super::app::App;
use super::tabs::Tab;
use super::widgets;
use ratatui::{
    layout::{Constraint, Direction, Layout},
    Frame,
};

/// Render the TUI interface
pub fn render(app: &App, frame: &mut Frame) {
    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Length(1), // Header
            Constraint::Min(0),    // Main content
            Constraint::Length(1), // Footer
        ])
        .split(frame.area());

    // Render header
    widgets::header::render(frame, chunks[0], app);

    // Render content based on current tab
    match app.current_tab {
        Tab::Console => widgets::console::render(frame, chunks[1], app),
        Tab::Position => widgets::position::render(frame, chunks[1], app),
        Tab::Jobs => widgets::jobs::render(frame, chunks[1], app),
        Tab::Help => widgets::help::render(frame, chunks[1], app),
    }

    // Render footer
    widgets::footer::render(frame, chunks[2], app);
}
