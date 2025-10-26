use crate::tui::app::App;
use ratatui::{
    layout::{Alignment, Constraint, Direction, Layout, Rect},
    style::{Color, Modifier, Style},
    text::{Line, Span},
    widgets::{Block, Borders, List, ListItem, Paragraph},
    Frame,
};

pub fn render(frame: &mut Frame, area: Rect, app: &App) {
    // Add temperature and position lines at top if enabled
    let mut constraints = vec![];
    if app.panel_visibility.temperature {
        constraints.push(Constraint::Length(1));
    }
    if app.panel_visibility.position {
        constraints.push(Constraint::Length(1));
    }
    constraints.push(Constraint::Min(0)); // Rest of jobs panel
    
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
        .title(" Print Jobs ")
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
            Line::from(""),
            Line::from(Span::styled(
                "Connect to view job history",
                Style::default().fg(Color::Gray),
            )),
        ];
        let paragraph = Paragraph::new(content)
            .block(block)
            .alignment(Alignment::Center);
        frame.render_widget(paragraph, main_area);
        return;
    }

    if app.job_list.items.is_empty() {
        let content = vec![
            Line::from(""),
            Line::from(Span::styled(
                "No print jobs found",
                Style::default().fg(Color::Yellow),
            )),
            Line::from(""),
            Line::from("Upload gcode files to see them here"),
            Line::from(""),
            Line::from(Span::styled(
                "Fetching jobs from Moonraker...",
                Style::default().fg(Color::Gray).add_modifier(Modifier::ITALIC),
            )),
        ];
        let paragraph = Paragraph::new(content)
            .block(block)
            .alignment(Alignment::Center);
        frame.render_widget(paragraph, main_area);
        return;
    }

    // Create list items from jobs (files)
    let items: Vec<ListItem> = app
        .job_list
        .items
        .iter()
        .map(|job| {
            // Format estimated time if available
            let duration = if job.total_duration > 0.0 {
                format_duration(job.total_duration)
            } else {
                "N/A".to_string()
            };
            
            // Format filament if available
            let filament = if job.filament_used > 0.0 {
                format!("{:.1}mm", job.filament_used)
            } else {
                "N/A".to_string()
            };

            let content = Line::from(vec![
                Span::styled("📄 ", Style::default().fg(Color::White)),
                Span::styled(&job.filename, Style::default().fg(Color::Cyan)),
                Span::raw(format!(" │ Est: {}", duration)),
                Span::raw(format!(" │ Fil: {}", filament)),
            ]);

            ListItem::new(content)
        })
        .collect();

    let list = List::new(items)
        .block(block)
        .highlight_style(
            Style::default()
                .bg(Color::DarkGray)
                .add_modifier(Modifier::BOLD),
        )
        .highlight_symbol("▶ ");

    frame.render_stateful_widget(list, main_area, &mut app.job_list.state.clone());

    // Show help text at bottom if there are jobs
    if !app.job_list.items.is_empty() {
        let help_text = " ↑↓ Navigate │ Enter Start Print │ Press 'm' for Main ";
        let help_area = Rect {
            x: main_area.x + 1,
            y: main_area.y + main_area.height - 2,
            width: main_area.width - 2,
            height: 1,
        };
        
        let help = Paragraph::new(Line::from(vec![
            Span::styled(help_text, Style::default().fg(Color::DarkGray)),
        ]));
        
        frame.render_widget(help, help_area);
    }
}

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
