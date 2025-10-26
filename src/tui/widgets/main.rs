use crate::tui::app::App;
use crate::tui::widgets;
use ratatui::{
    layout::{Alignment, Constraint, Direction, Layout, Rect},
    style::{Color, Modifier, Style},
    text::{Line, Span},
    widgets::{Block, Borders, Paragraph},
    Frame,
};

#[allow(dead_code)]
pub fn render(frame: &mut Frame, area: Rect, app: &App) {
    // Calculate dynamic constraints based on panel visibility
    let mut constraints = vec![];
    
    // Temperature line (1 line if visible)
    if app.panel_visibility.temperature {
        constraints.push(Constraint::Length(1));
    }
    
    // Position line (1 line if visible)
    if app.panel_visibility.position {
        constraints.push(Constraint::Length(1));
    }
    
    // Main info area (rest of space)
    constraints.push(Constraint::Min(5));
    
    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints(constraints)
        .split(area);

    let mut chunk_idx = 0;
    
    // Render temperature line if visible
    if app.panel_visibility.temperature {
        widgets::temperatures::render(frame, chunks[chunk_idx], app);
        chunk_idx += 1;
    }
    
    // Render position line if visible
    if app.panel_visibility.position {
        widgets::position_bar::render(frame, chunks[chunk_idx], app);
        chunk_idx += 1;
    }

    // Render main info area
    let block = Block::default()
        .title(" Main Dashboard ")
        .title_alignment(Alignment::Center)
        .borders(Borders::ALL)
        .border_style(Style::default().fg(Color::Cyan));

    let mut content = vec![
        Line::from(""),
        Line::from(Span::styled(
            "Main Dashboard",
            Style::default().fg(Color::Cyan).add_modifier(Modifier::BOLD),
        )),
        Line::from(""),
    ];

    // Show connection status
    if app.printer.connected {
        content.push(Line::from(Span::styled(
            format!("✓ Connected to {}", app.server_url),
            Style::default().fg(Color::Green),
        )));
        
        // Show print status if printing
        if app.printer.print_stats.state == "printing" {
            content.push(Line::from(""));
            content.push(Line::from(Span::styled(
                format!("Printing: {}", app.printer.print_stats.filename),
                Style::default().fg(Color::Cyan).add_modifier(Modifier::BOLD),
            )));
            content.push(Line::from(format!(
                "  Duration: {:.1}s",
                app.printer.print_stats.print_duration
            )));
            content.push(Line::from(format!(
                "  Filament: {:.2}mm",
                app.printer.print_stats.filament_used
            )));
        }
    } else {
        content.push(Line::from(Span::styled(
            format!("✗ Not connected to {}", app.server_url),
            Style::default().fg(Color::Red),
        )));
        content.push(Line::from(""));
        content.push(Line::from("Waiting for connection..."));
    }

    content.push(Line::from(""));
    content.push(Line::from("Panel Controls:"));
    content.push(Line::from(Span::styled(
        if app.panel_visibility.temperature {
            "  [t] Temperature panel: ON"
        } else {
            "  [t] Temperature panel: OFF"
        },
        Style::default().fg(if app.panel_visibility.temperature {
            Color::Green
        } else {
            Color::Gray
        }),
    )));
    content.push(Line::from(Span::styled(
        if app.panel_visibility.position {
            "  [p] Position bar: ON"
        } else {
            "  [p] Position bar: OFF"
        },
        Style::default().fg(if app.panel_visibility.position {
            Color::Green
        } else {
            Color::Gray
        }),
    )));

    let paragraph = Paragraph::new(content)
        .block(block)
        .alignment(Alignment::Left);

    frame.render_widget(paragraph, chunks[chunk_idx]);
}
