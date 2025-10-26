use crate::tui::app::App;
use ratatui::{
    layout::Rect,
    style::{Color, Modifier, Style},
    text::{Line, Span},
    widgets::Paragraph,
    Frame,
};

/// Render the header with connection status, printer state, and power devices
/// Returns the rectangles for power device click areas
pub fn render(frame: &mut Frame, area: Rect, app: &App) -> Vec<(Rect, String)> {
    let mut spans = vec![];
    let mut click_areas = vec![];

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
        
        // Power devices - clickable to toggle
        if !app.printer.power_devices.is_empty() {
            spans.push(Span::styled(
                " | ",
                Style::default().fg(Color::White),
            ));
            
            // Calculate current text width to know where power devices start
            let mut current_width: usize = spans.iter()
                .map(|s| s.content.len())
                .sum();
            
            for (i, device) in app.printer.power_devices.iter().enumerate() {
                if i > 0 {
                    let space_span = Span::styled(
                        " ",
                        Style::default().fg(Color::White),
                    );
                    current_width += space_span.content.len();
                    spans.push(space_span);
                }
                
                // Color code by status
                let device_color = match device.status.as_str() {
                    "on" => Color::Green,
                    "off" => Color::Gray,
                    "init" => Color::Yellow,
                    "error" => Color::Red,
                    _ => Color::White,
                };
                
                let device_text = format!("⚡{}", device.name);
                let device_span = Span::styled(
                    device_text.clone(),
                    Style::default()
                        .fg(device_color)
                        .add_modifier(Modifier::BOLD)
                        .add_modifier(Modifier::UNDERLINED),
                );
                
                // Calculate the click area for this device
                let span_width = device_text.len();
                
                // Store click area (x, y, width, height, device_name)
                click_areas.push((
                    Rect {
                        x: area.x + current_width as u16,
                        y: area.y,
                        width: span_width as u16,
                        height: 1,
                    },
                    device.name.clone(),
                ));
                
                current_width += span_width;
                spans.push(device_span);
            }
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
    
    click_areas
}

