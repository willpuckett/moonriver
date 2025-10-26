use crate::tui::app::App;
use ratatui::{
    layout::{Alignment, Rect},
    style::{Color, Modifier, Style},
    text::{Line, Span},
    widgets::{Block, Borders, Paragraph, Wrap},
    Frame,
};

pub fn render(frame: &mut Frame, area: Rect, app: &App) {
    let block = Block::default()
        .title(" Moonriver Help ")
        .title_alignment(Alignment::Center)
        .borders(Borders::ALL)
        .border_style(Style::default().fg(Color::Cyan));

    let mut content = vec![
        Line::from(""),
        Line::from(Span::styled(
            "Moonriver TUI - Keyboard Shortcuts",
            Style::default().fg(Color::Cyan).add_modifier(Modifier::BOLD),
        )),
        Line::from(""),
        Line::from(Span::styled("Global Keys:", Style::default().fg(Color::Yellow).add_modifier(Modifier::BOLD))),
        Line::from("  m - Main dashboard"),
        Line::from("  c - Console (GCode terminal)"),
        Line::from("  p - Position (toolhead & homing)"),
        Line::from("  j - Jobs (print history)"),
        Line::from("  h or ? - This help screen"),
        Line::from("  q - Quit application"),
        Line::from("  Ctrl+C - Emergency stop (if connected)"),
        Line::from(""),
        Line::from(Span::styled("Main Tab:", Style::default().fg(Color::Yellow).add_modifier(Modifier::BOLD))),
        Line::from("  t - Toggle temperature panel"),
        Line::from("  s - Toggle system info panel"),
        Line::from(""),
        Line::from(Span::styled("Console Tab:", Style::default().fg(Color::Yellow).add_modifier(Modifier::BOLD))),
        Line::from("  i - Enter editing mode"),
        Line::from("  Enter - Send command"),
        Line::from("  Esc - Cancel editing"),
        Line::from("  ↑↓ - Navigate command history"),
        Line::from(""),
        Line::from(Span::styled("Position Tab:", Style::default().fg(Color::Yellow).add_modifier(Modifier::BOLD))),
        Line::from("  x - Home X axis"),
        Line::from("  y - Home Y axis"),
        Line::from("  z - Home Z axis"),
        Line::from("  a - Home all axes"),
        Line::from(""),
        Line::from(Span::styled("Jobs Tab:", Style::default().fg(Color::Yellow).add_modifier(Modifier::BOLD))),
        Line::from("  ↑↓ or j/k - Navigate job list"),
        Line::from("  Enter - Start selected print job"),
        Line::from("  r - Refresh job list"),
        Line::from(""),
        Line::from(Span::styled("Features:", Style::default().fg(Color::Green).add_modifier(Modifier::BOLD))),
        Line::from("  ✓ Real-time temperature monitoring"),
        Line::from("  ✓ Live position tracking"),
        Line::from("  ✓ GCode console with command history"),
        Line::from("  ✓ Print status and job info"),
        Line::from("  ✓ Connection status indicators"),
    ];

    // Add context-specific help based on current tab
    content.push(Line::from(""));
    content.push(Line::from(Span::styled(
        format!("Current Tab: {}", app.current_tab.name()),
        Style::default().fg(Color::Cyan).add_modifier(Modifier::ITALIC),
    )));

    let paragraph = Paragraph::new(content)
        .block(block)
        .alignment(Alignment::Left)
        .wrap(Wrap { trim: false });

    frame.render_widget(paragraph, area);
}
