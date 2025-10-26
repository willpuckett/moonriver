use crate::tui::app::App;
use ratatui::{
    layout::Rect,
    style::{Color, Modifier, Style},
    text::{Line, Span},
    widgets::Paragraph,
    Frame,
};

pub fn render(frame: &mut Frame, area: Rect, app: &App) {
    let mut spans = vec![];

    // Connection status
    let (status_char, status_color) = if app.printer.connected {
        ("✔", Color::Green)
    } else {
        ("✕", Color::Red)
    };

    spans.push(Span::styled(
        format!(" {} ", status_char),
        Style::default().fg(Color::White).bg(status_color).add_modifier(Modifier::BOLD),
    ));

    // Server URL
    spans.push(Span::styled(
        format!(" {} ", app.server_url),
        Style::default().fg(Color::White),
    ));

    // Connection and Printer state
    if !app.printer.connected {
        // Show disconnected status
        spans.push(Span::styled(
            " | Disconnected ",
            Style::default().fg(Color::Red).add_modifier(Modifier::BOLD),
        ));
    } else {
        // Show printer state from print_stats (more accurate than the generic state field)
        let state_text = &app.printer.print_stats.state;
        let (display_text, state_color) = match state_text.as_str() {
            "standby" => ("Standby", Color::Gray),
            "printing" => ("Printing", Color::Green),
            "paused" => ("Paused", Color::Yellow),
            "complete" => ("Complete", Color::Green),
            "cancelled" => ("Cancelled", Color::Yellow),
            "error" => ("Error", Color::Red),
            _ => ("Connected", Color::Green),
        };
        
        spans.push(Span::styled(
            format!(" | {} ", display_text),
            Style::default().fg(state_color).add_modifier(Modifier::BOLD),
        ));
        
        // Show filename if printing or just completed
        if !app.printer.print_stats.filename.is_empty() {
            spans.push(Span::styled(
                format!(" | {} ", app.printer.print_stats.filename),
                Style::default().fg(Color::Cyan),
            ));
        }
        
        // Show duration if printing
        if state_text == "printing" && app.printer.print_stats.print_duration > 0.0 {
            let duration_secs = app.printer.print_stats.print_duration as u64;
            let hours = duration_secs / 3600;
            let minutes = (duration_secs % 3600) / 60;
            let seconds = duration_secs % 60;
            
            spans.push(Span::styled(
                format!(" | {:02}:{:02}:{:02} ", hours, minutes, seconds),
                Style::default().fg(Color::Yellow),
            ));
        }
    }

    // Current tab indicator
    spans.push(Span::styled(
        format!(" | {} ", app.current_tab.name()),
        Style::default().fg(Color::Magenta),
    ));

    let header = Paragraph::new(Line::from(spans))
        .style(Style::default().bg(Color::Black));

    frame.render_widget(header, area);
}
