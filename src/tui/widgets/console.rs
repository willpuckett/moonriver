use crate::tui::app::{App, ConsoleMessage, InputMode};
use ratatui::{
    layout::{Alignment, Constraint, Direction, Layout, Rect},
    style::{Color, Modifier, Style},
    text::{Line, Span},
    widgets::{Block, Borders, Paragraph},
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
    constraints.push(Constraint::Min(0)); // Rest of console
    
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
    
    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Min(3),    // Output area
            Constraint::Length(3), // Input area
        ])
        .split(main_area);

    // Output area - show console messages
    let output_block = Block::default()
        .title(" GCode Console ")
        .title_alignment(Alignment::Center)
        .borders(Borders::ALL)
        .border_style(Style::default().fg(Color::Cyan));

    // Build output content from message history
    let mut output_content: Vec<Line> = vec![];
    
    if app.console_messages.is_empty() {
        output_content.push(Line::from(""));
        output_content.push(Line::from(Span::styled(
            "GCode Console",
            Style::default().fg(Color::Cyan).add_modifier(Modifier::BOLD),
        )));
        output_content.push(Line::from(""));
        output_content.push(Line::from("Press 'i' to enter a command"));
        output_content.push(Line::from(""));
        if !app.printer.connected {
            output_content.push(Line::from(Span::styled(
                "⚠ Not connected to printer",
                Style::default().fg(Color::Yellow),
            )));
        } else {
            output_content.push(Line::from(Span::styled(
                "✓ Connected to printer",
                Style::default().fg(Color::Green),
            )));
        }
    } else {
        // Show recent messages (limit to avoid overwhelming the display)
        let start_idx = app.console_messages.len().saturating_sub(100);
        for msg in &app.console_messages[start_idx..] {
            match msg {
                ConsoleMessage::Command(cmd) => {
                    output_content.push(Line::from(vec![
                        Span::styled("> ", Style::default().fg(Color::Green).add_modifier(Modifier::BOLD)),
                        Span::styled(cmd, Style::default().fg(Color::White)),
                    ]));
                }
                ConsoleMessage::Response(resp) => {
                    output_content.push(Line::from(Span::styled(
                        resp,
                        Style::default().fg(Color::Cyan),
                    )));
                }
                ConsoleMessage::Error(err) => {
                    output_content.push(Line::from(Span::styled(
                        format!("✗ {}", err),
                        Style::default().fg(Color::Red).add_modifier(Modifier::BOLD),
                    )));
                }
                ConsoleMessage::Info(info) => {
                    output_content.push(Line::from(Span::styled(
                        format!("  {}", info),
                        Style::default().fg(Color::Gray),
                    )));
                }
            }
        }
    }

    let output = Paragraph::new(output_content.clone())
        .block(output_block)
        .alignment(Alignment::Left)
        .scroll((app.console_scroll, 0));

    frame.render_widget(output, chunks[0]);

    // Auto-scroll to bottom: calculate how many lines to scroll
    // to show the most recent content
    let output_height = chunks[0].height.saturating_sub(2); // Subtract borders
    let total_lines = output_content.len() as u16;
    
    // If content exceeds visible area and scroll is not manually controlled,
    // we should auto-scroll to show the bottom
    // Note: We're using the stored scroll value, which should be updated
    // when new messages arrive (handled in app logic)
    if total_lines > output_height {
        // Content is longer than display area - needs scrolling
        // The app.console_scroll value determines what's visible
        // For auto-scroll behavior, see app's message handling
    }

    // Input area
    let input_style = match app.console_input.mode {
        InputMode::Normal => Style::default().fg(Color::Gray),
        InputMode::Editing => Style::default().fg(Color::Yellow),
    };

    let input_block = Block::default()
        .title(match app.console_input.mode {
            InputMode::Normal => " Input (press 'i' to edit) ",
            InputMode::Editing => " Input (press Esc to exit, Enter to send) ",
        })
        .borders(Borders::ALL)
        .border_style(input_style);

    let input = Paragraph::new(app.console_input.value.as_str())
        .style(input_style)
        .block(input_block);

    frame.render_widget(input, chunks[1]);

    // Show cursor when editing
    if app.console_input.mode == InputMode::Editing {
        frame.set_cursor_position((
            chunks[1].x + app.console_input.cursor_position + 1,
            chunks[1].y + 1,
        ));
    }
}
