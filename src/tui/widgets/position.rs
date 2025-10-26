use crate::tui::app::App;
use ratatui::{
    layout::{Alignment, Constraint, Direction, Layout, Rect},
    style::{Color, Modifier, Style},
    text::{Line, Span},
    widgets::{Block, Borders, Paragraph},
    Frame,
};

pub fn render(frame: &mut Frame, area: Rect, app: &App) {
    // Add temperature and position bars at top if enabled
    let mut constraints = vec![];
    if app.panel_visibility.temperature {
        constraints.push(Constraint::Length(1));
    }
    if app.panel_visibility.position {
        constraints.push(Constraint::Length(1));
    }
    constraints.push(Constraint::Min(0)); // Rest of position panel
    
    let main_area = if app.panel_visibility.temperature || app.panel_visibility.position {
        let chunks = Layout::default()
            .direction(Direction::Vertical)
            .constraints(constraints)
            .split(area);
        
        let mut chunk_idx = 0;
        if app.panel_visibility.temperature {
            super::temperatures::render(frame, chunks[chunk_idx], app);
            chunk_idx += 1;
        }
        if app.panel_visibility.position {
            super::position_bar::render(frame, chunks[chunk_idx], app);
            chunk_idx += 1;
        }
        chunks[chunk_idx]
    } else {
        area
    };
    
    let block = Block::default()
        .title(" Position Control ")
        .title_alignment(Alignment::Center)
        .borders(Borders::ALL)
        .border_style(Style::default().fg(Color::Cyan));

    if !app.printer.connected {
        let content = vec![
            Line::from(""),
            Line::from(Span::styled(
                "Not connected to printer",
                Style::default().fg(Color::Red),
            )),
        ];
        let paragraph = Paragraph::new(content)
            .block(block)
            .alignment(Alignment::Center);
        frame.render_widget(paragraph, main_area);
        return;
    }

    // Split area into position display and controls
    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Length(10), // Position display
            Constraint::Min(5),     // Controls
        ])
        .split(main_area);

    // Render position display
    render_position_display(frame, chunks[0], app);

    // Render controls
    render_controls(frame, chunks[1], app);
}

fn render_position_display(frame: &mut Frame, area: Rect, app: &App) {
    let block = Block::default()
        .title(" Current Position ")
        .borders(Borders::ALL)
        .border_style(Style::default().fg(Color::Green));

    let position = &app.printer.toolhead.position;
    let homed_axes = &app.printer.toolhead.homed_axes;

    let x_homed = homed_axes.contains(&"x".to_string());
    let y_homed = homed_axes.contains(&"y".to_string());
    let z_homed = homed_axes.contains(&"z".to_string());

    let homed_indicator = |homed: bool| {
        if homed {
            Span::styled("✓", Style::default().fg(Color::Green))
        } else {
            Span::styled("✗", Style::default().fg(Color::Red))
        }
    };

    let content = vec![
        Line::from(""),
        Line::from(vec![
            homed_indicator(x_homed),
            Span::raw(" X: "),
            Span::styled(
                format!("{:>8.2} mm", position[0]),
                Style::default().fg(Color::Cyan).add_modifier(Modifier::BOLD),
            ),
        ]),
        Line::from(vec![
            homed_indicator(y_homed),
            Span::raw(" Y: "),
            Span::styled(
                format!("{:>8.2} mm", position[1]),
                Style::default().fg(Color::Cyan).add_modifier(Modifier::BOLD),
            ),
        ]),
        Line::from(vec![
            homed_indicator(z_homed),
            Span::raw(" Z: "),
            Span::styled(
                format!("{:>8.2} mm", position[2]),
                Style::default().fg(Color::Cyan).add_modifier(Modifier::BOLD),
            ),
        ]),
        Line::from(vec![
            Span::raw("   E: "),
            Span::styled(
                format!("{:>8.2} mm", position[3]),
                Style::default().fg(Color::Cyan).add_modifier(Modifier::BOLD),
            ),
        ]),
    ];

    let paragraph = Paragraph::new(content)
        .block(block)
        .alignment(Alignment::Left);

    frame.render_widget(paragraph, area);
}

fn render_controls(frame: &mut Frame, area: Rect, _app: &App) {
    let block = Block::default()
        .title(" Movement Controls ")
        .borders(Borders::ALL)
        .border_style(Style::default().fg(Color::Yellow));

    let content = vec![
        Line::from(""),
        Line::from(Span::styled(
            "Homing Controls",
            Style::default().fg(Color::Yellow).add_modifier(Modifier::BOLD),
        )),
        Line::from(""),
        Line::from("  x - Home X axis"),
        Line::from("  y - Home Y axis"),
        Line::from("  z - Home Z axis"),
        Line::from("  a - Home all axes"),
        Line::from(""),
        Line::from(Span::styled(
            "Jogging (Coming Soon)",
            Style::default().fg(Color::Gray).add_modifier(Modifier::ITALIC),
        )),
        Line::from("  Arrow keys - Jog X/Y"),
        Line::from("  PgUp/PgDn - Jog Z"),
        Line::from("  +/- - Change distance"),
    ];

    let paragraph = Paragraph::new(content)
        .block(block)
        .alignment(Alignment::Left);

    frame.render_widget(paragraph, area);
}
