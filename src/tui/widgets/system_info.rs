use crate::tui::app::App;
use ratatui::{
    layout::{Alignment, Constraint, Layout, Rect},
    style::{Color, Modifier, Style},
    text::{Line, Span},
    widgets::{Block, Borders, Paragraph},
    Frame,
};

#[allow(dead_code)]
pub fn render(frame: &mut Frame, area: Rect, app: &App) {
    let block = Block::default()
        .title(" System Info ")
        .borders(Borders::ALL)
        .border_style(Style::default().fg(Color::Blue));

    if !app.printer.connected {
        let content = vec![
            Line::from(""),
            Line::from(Span::styled(
                "Not connected",
                Style::default().fg(Color::Gray),
            )),
        ];
        let paragraph = Paragraph::new(content)
            .block(block)
            .alignment(Alignment::Center);
        frame.render_widget(paragraph, area);
        return;
    }

    // Split into two columns for more information density
    let inner = block.inner(area);
    frame.render_widget(block, area);
    
    let columns = Layout::default()
        .direction(ratatui::layout::Direction::Horizontal)
        .constraints([
            Constraint::Percentage(50),
            Constraint::Percentage(50),
        ])
        .split(inner);

    // Left column - Printer Status & State
    let mut left_content = vec![];
    
    // Printer state with color coding
    let state_style = match app.printer.print_stats.state.as_str() {
        "standby" => Style::default().fg(Color::Gray),
        "printing" => Style::default().fg(Color::Green).add_modifier(Modifier::BOLD),
        "paused" => Style::default().fg(Color::Yellow).add_modifier(Modifier::BOLD),
        "complete" => Style::default().fg(Color::Green),
        "cancelled" => Style::default().fg(Color::Yellow),
        "error" => Style::default().fg(Color::Red).add_modifier(Modifier::BOLD),
        _ => Style::default().fg(Color::White),
    };
    
    left_content.push(Line::from(vec![
        Span::styled("State: ", Style::default().fg(Color::Gray)),
        Span::styled(&app.printer.print_stats.state, state_style),
    ]));
    
    // Connection status
    let conn_status = if app.printer.connected {
        ("Connected", Color::Green)
    } else {
        ("Disconnected", Color::Red)
    };
    left_content.push(Line::from(vec![
        Span::styled("Klipper: ", Style::default().fg(Color::Gray)),
        Span::styled(conn_status.0, Style::default().fg(conn_status.1).add_modifier(Modifier::BOLD)),
    ]));
    
    // Print file if printing
    if !app.printer.print_stats.filename.is_empty() {
        left_content.push(Line::from(""));
        left_content.push(Line::from(vec![
            Span::styled("File: ", Style::default().fg(Color::Gray)),
            Span::styled(&app.printer.print_stats.filename, Style::default().fg(Color::Cyan)),
        ]));
        
        // Print duration
        if app.printer.print_stats.print_duration > 0.0 {
            let duration = format_duration(app.printer.print_stats.print_duration);
            left_content.push(Line::from(vec![
                Span::styled("Duration: ", Style::default().fg(Color::Gray)),
                Span::styled(duration, Style::default().fg(Color::White)),
            ]));
        }
        
        // Filament used
        if app.printer.print_stats.filament_used > 0.0 {
            left_content.push(Line::from(vec![
                Span::styled("Filament: ", Style::default().fg(Color::Gray)),
                Span::styled(
                    format!("{:.1}mm", app.printer.print_stats.filament_used),
                    Style::default().fg(Color::White)
                ),
            ]));
        }
    }
    
    let left_para = Paragraph::new(left_content)
        .alignment(Alignment::Left);
    frame.render_widget(left_para, columns[0]);
    
    // Right column - Toolhead & Position Info
    let mut right_content = vec![];
    
    // Toolhead position (simplified)
    let pos = &app.printer.toolhead.position;
    right_content.push(Line::from(vec![
        Span::styled("Position: ", Style::default().fg(Color::Gray)),
    ]));
    right_content.push(Line::from(vec![
        Span::styled(" X:", Style::default().fg(Color::DarkGray)),
        Span::styled(
            format!("{:.1}", pos[0]),
            Style::default().fg(Color::Cyan)
        ),
        Span::styled(" Y:", Style::default().fg(Color::DarkGray)),
        Span::styled(
            format!("{:.1}", pos[1]),
            Style::default().fg(Color::Cyan)
        ),
        Span::styled(" Z:", Style::default().fg(Color::DarkGray)),
        Span::styled(
            format!("{:.1}", pos[2]),
            Style::default().fg(Color::Cyan)
        ),
    ]));
    
    // Homed axes
    let homed = &app.printer.toolhead.homed_axes;
    let homed_x = homed.contains(&"x".to_string());
    let homed_y = homed.contains(&"y".to_string());
    let homed_z = homed.contains(&"z".to_string());
    
    right_content.push(Line::from(vec![
        Span::styled("Homed: ", Style::default().fg(Color::Gray)),
        Span::styled(
            if homed_x { "X" } else { "x" },
            if homed_x { 
                Style::default().fg(Color::Green).add_modifier(Modifier::BOLD) 
            } else { 
                Style::default().fg(Color::DarkGray) 
            }
        ),
        Span::raw(" "),
        Span::styled(
            if homed_y { "Y" } else { "y" },
            if homed_y { 
                Style::default().fg(Color::Green).add_modifier(Modifier::BOLD) 
            } else { 
                Style::default().fg(Color::DarkGray) 
            }
        ),
        Span::raw(" "),
        Span::styled(
            if homed_z { "Z" } else { "z" },
            if homed_z { 
                Style::default().fg(Color::Green).add_modifier(Modifier::BOLD) 
            } else { 
                Style::default().fg(Color::DarkGray) 
            }
        ),
    ]));
    
    // Server connection
    right_content.push(Line::from(""));
    right_content.push(Line::from(vec![
        Span::styled("Server: ", Style::default().fg(Color::Gray)),
    ]));
    right_content.push(Line::from(vec![
        Span::styled(
            &app.server_url,
            Style::default().fg(Color::DarkGray)
        ),
    ]));

    let right_para = Paragraph::new(right_content)
        .alignment(Alignment::Left);
    frame.render_widget(right_para, columns[1]);
}

#[allow(dead_code)]
fn format_duration(seconds: f64) -> String {
    let secs = seconds as u64;
    let hours = secs / 3600;
    let minutes = (secs % 3600) / 60;
    let seconds = secs % 60;
    
    if hours > 0 {
        format!("{}h {:02}m {:02}s", hours, minutes, seconds)
    } else if minutes > 0 {
        format!("{}m {:02}s", minutes, seconds)
    } else {
        format!("{}s", seconds)
    }
}
