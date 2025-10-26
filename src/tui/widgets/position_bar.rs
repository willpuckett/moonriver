use crate::tui::app::App;
use ratatui::{
    layout::Rect,
    style::{Color, Modifier, Style},
    text::{Line, Span},
    widgets::Paragraph,
    Frame,
};

/// Render a compact single-line position display
pub fn render(frame: &mut Frame, area: Rect, app: &App) {
    if !app.panel_visibility.position {
        return;
    }

    let mut spans = vec![
        Span::styled("📍 ", Style::default().fg(Color::White)),
    ];

    let position = &app.printer.toolhead.position;
    let homed_axes = &app.printer.toolhead.homed_axes;

    let x_homed = homed_axes.contains(&"x".to_string());
    let y_homed = homed_axes.contains(&"y".to_string());
    let z_homed = homed_axes.contains(&"z".to_string());

    // Check if any axis is being edited
    let editing_x = app.pos_edit_target == Some(crate::tui::app::PosEditTarget::X) 
        && app.pos_input.mode == crate::tui::app::InputMode::Editing;
    let editing_y = app.pos_edit_target == Some(crate::tui::app::PosEditTarget::Y) 
        && app.pos_input.mode == crate::tui::app::InputMode::Editing;
    let editing_z = app.pos_edit_target == Some(crate::tui::app::PosEditTarget::Z) 
        && app.pos_input.mode == crate::tui::app::InputMode::Editing;

    // X axis
    spans.extend(vec![
        Span::styled(
            if x_homed { "✓" } else { "✗" },
            Style::default().fg(if x_homed { Color::Green } else { Color::Red }),
        ),
        Span::styled("X:", Style::default().fg(Color::Cyan).add_modifier(Modifier::BOLD)),
    ]);
    
    if editing_x {
        spans.push(Span::styled(
            format!("[{}]", if app.pos_input.value.is_empty() { "_" } else { &app.pos_input.value }),
            Style::default().fg(Color::Yellow).add_modifier(Modifier::BOLD | Modifier::UNDERLINED),
        ));
    } else {
        spans.push(Span::styled(
            format!("{:.1}", position[0]),
            Style::default().fg(if x_homed { Color::Cyan } else { Color::DarkGray }),
        ));
    }

    // Y axis
    spans.extend(vec![
        Span::raw("  "),
        Span::styled(
            if y_homed { "✓" } else { "✗" },
            Style::default().fg(if y_homed { Color::Green } else { Color::Red }),
        ),
        Span::styled("Y:", Style::default().fg(Color::Cyan).add_modifier(Modifier::BOLD)),
    ]);
    
    if editing_y {
        spans.push(Span::styled(
            format!("[{}]", if app.pos_input.value.is_empty() { "_" } else { &app.pos_input.value }),
            Style::default().fg(Color::Yellow).add_modifier(Modifier::BOLD | Modifier::UNDERLINED),
        ));
    } else {
        spans.push(Span::styled(
            format!("{:.1}", position[1]),
            Style::default().fg(if y_homed { Color::Cyan } else { Color::DarkGray }),
        ));
    }

    // Z axis
    spans.extend(vec![
        Span::raw("  "),
        Span::styled(
            if z_homed { "✓" } else { "✗" },
            Style::default().fg(if z_homed { Color::Green } else { Color::Red }),
        ),
        Span::styled("Z:", Style::default().fg(Color::Cyan).add_modifier(Modifier::BOLD)),
    ]);
    
    if editing_z {
        spans.push(Span::styled(
            format!("[{}]", if app.pos_input.value.is_empty() { "_" } else { &app.pos_input.value }),
            Style::default().fg(Color::Yellow).add_modifier(Modifier::BOLD | Modifier::UNDERLINED),
        ));
    } else {
        spans.push(Span::styled(
            format!("{:.2}", position[2]),
            Style::default().fg(if z_homed { Color::Cyan } else { Color::DarkGray }),
        ));
    }

    // Add home all button
    spans.extend(vec![
        Span::raw("  │  "),
        Span::styled(
            "🏠",
            Style::default().fg(Color::Yellow),
        ),
        Span::styled(
            " Home All",
            Style::default().fg(Color::Yellow).add_modifier(Modifier::BOLD),
        ),
    ]);

    let line = Line::from(spans);
    let paragraph = Paragraph::new(line);
    frame.render_widget(paragraph, area);
}

/// Calculate clickable bounds for position values and home button
/// Returns (PosEditTarget or HomeAll, Rect) for each clickable element
pub fn get_position_bounds(area: Rect, app: &crate::tui::app::App) -> Vec<(PositionAction, Rect)> {
    use crate::tui::app::PosEditTarget;
    
    let mut bounds = Vec::new();
    let y = area.y;
    let mut x = area.x;
    
    // Helper function to measure text width
    let text_width = |text: &str| -> u16 {
        text.chars().map(|c| if c.is_ascii() { 1 } else { 2 }).sum::<usize>() as u16
    };
    
    let position = &app.printer.toolhead.position;
    let homed_axes = &app.printer.toolhead.homed_axes;
    
    let x_homed = homed_axes.contains(&"x".to_string());
    let y_homed = homed_axes.contains(&"y".to_string());
    let z_homed = homed_axes.contains(&"z".to_string());
    
    // Check if any axis is being edited
    let editing_x = app.pos_edit_target == Some(PosEditTarget::X) 
        && app.pos_input.mode == crate::tui::app::InputMode::Editing;
    let editing_y = app.pos_edit_target == Some(PosEditTarget::Y) 
        && app.pos_input.mode == crate::tui::app::InputMode::Editing;
    let editing_z = app.pos_edit_target == Some(PosEditTarget::Z) 
        && app.pos_input.mode == crate::tui::app::InputMode::Editing;
    
    // "📍 " = 3 chars (emoji takes 2)
    x += text_width("📍 ");
    
    // X axis - build the actual text
    x += text_width(if x_homed { "✓" } else { "✗" });
    let x_clickable_start = x;
    
    let x_text = if editing_x {
        format!("X:[{}]", if app.pos_input.value.is_empty() { "_" } else { &app.pos_input.value })
    } else {
        format!("X:{:.1}", position[0])
    };
    let x_width = text_width(&x_text);
    x += x_width;
    
    bounds.push((
        PositionAction::EditAxis(PosEditTarget::X),
        Rect {
            x: x_clickable_start,
            y,
            width: x_width,
            height: 1,
        },
    ));
    
    // Y axis
    x += text_width("  "); // spacing
    x += text_width(if y_homed { "✓" } else { "✗" });
    let y_clickable_start = x;
    
    let y_text = if editing_y {
        format!("Y:[{}]", if app.pos_input.value.is_empty() { "_" } else { &app.pos_input.value })
    } else {
        format!("Y:{:.1}", position[1])
    };
    let y_width = text_width(&y_text);
    x += y_width;
    
    bounds.push((
        PositionAction::EditAxis(PosEditTarget::Y),
        Rect {
            x: y_clickable_start,
            y,
            width: y_width,
            height: 1,
        },
    ));
    
    // Z axis
    x += text_width("  "); // spacing
    x += text_width(if z_homed { "✓" } else { "✗" });
    let z_clickable_start = x;
    
    let z_text = if editing_z {
        format!("Z:[{}]", if app.pos_input.value.is_empty() { "_" } else { &app.pos_input.value })
    } else {
        format!("Z:{:.2}", position[2])
    };
    let z_width = text_width(&z_text);
    x += z_width;
    
    bounds.push((
        PositionAction::EditAxis(PosEditTarget::Z),
        Rect {
            x: z_clickable_start,
            y,
            width: z_width,
            height: 1,
        },
    ));
    
    // Home button: "  │  🏠 Home All"
    x += text_width("  │  ");
    let home_start = x;
    let home_text = "🏠 Home All";
    let home_width = text_width(home_text);
    
    bounds.push((
        PositionAction::HomeAll,
        Rect {
            x: home_start,
            y,
            width: home_width,
            height: 1,
        },
    ));
    
    bounds
}

/// Actions that can be performed on the position bar
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum PositionAction {
    EditAxis(crate::tui::app::PosEditTarget),
    HomeAll,
}
