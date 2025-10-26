use crate::tui::app::App;
use ratatui::{
    layout::Rect,
    style::{Color, Modifier, Style},
    text::{Line, Span},
    widgets::Paragraph,
    Frame,
};

/// Render a compact single-line temperature display
pub fn render(frame: &mut Frame, area: Rect, app: &App) {
    if !app.panel_visibility.temperature {
        return;
    }

    let mut spans = vec![
        Span::styled("🌡 ", Style::default().fg(Color::White)),
    ];

    // Extruder temperature
    let extruder = &app.printer.temperatures.extruder;
    let extruder_color = get_temp_color(extruder.temperature, extruder.target);
    
    // Check if extruder is being edited
    let editing_extruder = app.temp_edit_target == Some(crate::tui::app::TempEditTarget::Extruder) 
        && app.temp_input.mode == crate::tui::app::InputMode::Editing;
    
    spans.extend(vec![
        Span::styled("E:", Style::default().fg(Color::Magenta).add_modifier(Modifier::BOLD)),
        Span::styled(
            format!("{:.1}°", extruder.temperature),
            Style::default().fg(extruder_color),
        ),
        Span::raw("/"),
    ]);
    
    if editing_extruder {
        spans.push(Span::styled(
            format!("[{}°]", if app.temp_input.value.is_empty() { "_" } else { &app.temp_input.value }),
            Style::default().fg(Color::Yellow).add_modifier(Modifier::BOLD | Modifier::UNDERLINED),
        ));
    } else {
        spans.push(Span::styled(
            format!("{:.0}°", extruder.target),
            Style::default().fg(Color::DarkGray),
        ));
    }

    // Bed temperature
    let bed = &app.printer.temperatures.bed;
    let bed_color = get_temp_color(bed.temperature, bed.target);
    
    // Check if bed is being edited
    let editing_bed = app.temp_edit_target == Some(crate::tui::app::TempEditTarget::Bed) 
        && app.temp_input.mode == crate::tui::app::InputMode::Editing;
    
    spans.extend(vec![
        Span::raw("  "),
        Span::styled("🛌", Style::default().fg(Color::Red)),
        Span::styled(
            format!("{:.1}°", bed.temperature),
            Style::default().fg(bed_color),
        ),
        Span::raw("/"),
    ]);
    
    if editing_bed {
        spans.push(Span::styled(
            format!("[{}°]", if app.temp_input.value.is_empty() { "_" } else { &app.temp_input.value }),
            Style::default().fg(Color::Yellow).add_modifier(Modifier::BOLD | Modifier::UNDERLINED),
        ));
    } else {
        spans.push(Span::styled(
            format!("{:.0}°", bed.target),
            Style::default().fg(Color::DarkGray),
        ));
    }

    // Chamber temperature (if available)
    if let Some(chamber) = &app.printer.temperatures.chamber {
        let chamber_color = get_temp_color(chamber.temperature, chamber.target);
        spans.extend(vec![
            Span::raw("  "),
            Span::styled("C:", Style::default().fg(Color::Cyan).add_modifier(Modifier::BOLD)),
            Span::styled(
                format!("{:.1}°", chamber.temperature),
                Style::default().fg(chamber_color),
            ),
            Span::styled(
                format!("/{:.0}°", chamber.target),
                Style::default().fg(Color::DarkGray),
            ),
        ]);
    }

    // MCU temperatures (if available)
    for mcu in &app.printer.temperatures.mcus {
        // MCU temps don't have targets, so we use a simpler color scheme
        let mcu_color = if mcu.temperature < 50.0 {
            Color::Green
        } else if mcu.temperature < 70.0 {
            Color::Yellow
        } else if mcu.temperature < 85.0 {
            Color::Red
        } else {
            Color::Magenta // Very hot!
        };
        
        spans.extend(vec![
            Span::raw("  "),
            Span::styled("μC:", Style::default().fg(Color::Blue).add_modifier(Modifier::BOLD)),
            Span::styled(
                format!("{:.1}°", mcu.temperature),
                Style::default().fg(mcu_color),
            ),
        ]);
    }

    // Fan speeds (if available)
    for (fan_idx, fan) in app.printer.temperatures.fans.iter().enumerate() {
        let fan_percent = (fan.speed * 100.0) as u8;
        let fan_color = if fan.speed < 0.01 {
            Color::DarkGray
        } else if fan.speed < 0.5 {
            Color::Cyan
        } else {
            Color::Green
        };
        
        // Check if this fan is being edited
        let editing_fan = app.fan_edit_target
            .map(|target| target.index == fan_idx)
            .unwrap_or(false) && app.fan_input.mode == crate::tui::app::InputMode::Editing;
        
        spans.extend(vec![
            Span::raw("  "),
            Span::styled("🌀", Style::default().fg(Color::Cyan)),
        ]);
        
        if editing_fan {
            spans.push(Span::styled(
                format!("[{}%]", if app.fan_input.value.is_empty() { "_" } else { &app.fan_input.value }),
                Style::default().fg(Color::Yellow).add_modifier(Modifier::BOLD | Modifier::UNDERLINED),
            ));
        } else {
            spans.push(Span::styled(
                format!("{}%", fan_percent),
                Style::default().fg(fan_color),
            ));
        }
        
        // Show RPM if available
        if let Some(rpm) = fan.rpm {
            spans.push(Span::styled(
                format!("({:.0}rpm)", rpm),
                Style::default().fg(Color::DarkGray),
            ));
        }
    }

    let line = Line::from(spans);
    let paragraph = Paragraph::new(line);
    frame.render_widget(paragraph, area);
}

/// Get color based on how close temperature is to target
fn get_temp_color(current: f64, target: f64) -> Color {
    if target < 1.0 {
        // No target set, show as inactive
        return Color::DarkGray;
    }

    let diff = (target - current).abs();
    
    if diff < 2.0 {
        // Within 2°C - at temperature
        Color::Green
    } else if diff < 5.0 {
        // Within 5°C - approaching
        Color::Yellow
    } else if diff < 15.0 {
        // Within 15°C - heating/cooling
        Color::Cyan
    } else {
        // Far from target
        Color::White
    }
}

/// Clickable element in the temperature bar
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum TempBarElement {
    Extruder,
    Bed,
    Fan(usize), // Index of the fan in the fans array
}

/// Calculate clickable bounds for temperature setpoints and fans
/// Returns (TempBarElement, Rect) for each clickable element
pub fn get_temp_bounds(area: Rect, app: &crate::tui::app::App) -> Vec<(TempBarElement, Rect)> {
    let mut bounds = Vec::new();
    let y = area.y;
    let mut x = area.x;
    
    // Helper function to measure text width
    let text_width = |text: &str| -> u16 {
        text.chars().map(|c| if c.is_ascii() { 1 } else { 2 }).sum::<usize>() as u16
    };
    
    // "🌡 " = 3 chars (emoji takes 2)
    x += text_width("🌡 ");
    
    // Extruder section - build the actual text that will be rendered
    let extruder_start = x;
    let editing_extruder = app.temp_edit_target == Some(crate::tui::app::TempEditTarget::Extruder) 
        && app.temp_input.mode == crate::tui::app::InputMode::Editing;
    
    let extruder_text = if editing_extruder {
        format!("E:{:.1}°/[{}°]", 
            app.printer.temperatures.extruder.temperature,
            if app.temp_input.value.is_empty() { "_" } else { &app.temp_input.value }
        )
    } else {
        format!("E:{:.1}°/{:.0}°", 
            app.printer.temperatures.extruder.temperature,
            app.printer.temperatures.extruder.target
        )
    };
    
    let extruder_width = text_width(&extruder_text);
    x += extruder_width;
    
    bounds.push((
        TempBarElement::Extruder,
        Rect {
            x: extruder_start,
            y,
            width: extruder_width,
            height: 1,
        },
    ));
    
    // Bed section
    x += text_width("  "); // spacing
    let bed_start = x;
    let editing_bed = app.temp_edit_target == Some(crate::tui::app::TempEditTarget::Bed) 
        && app.temp_input.mode == crate::tui::app::InputMode::Editing;
    
    let bed_text = if editing_bed {
        format!("🛌{:.1}°/[{}°]", 
            app.printer.temperatures.bed.temperature,
            if app.temp_input.value.is_empty() { "_" } else { &app.temp_input.value }
        )
    } else {
        format!("🛌{:.1}°/{:.0}°", 
            app.printer.temperatures.bed.temperature,
            app.printer.temperatures.bed.target
        )
    };
    
    let bed_width = text_width(&bed_text);
    x += bed_width;
    
    bounds.push((
        TempBarElement::Bed,
        Rect {
            x: bed_start,
            y,
            width: bed_width,
            height: 1,
        },
    ));
    
    // Chamber temperature (if available)
    if let Some(chamber) = &app.printer.temperatures.chamber {
        let chamber_text = format!("  C:{:.1}°/{:.0}°", chamber.temperature, chamber.target);
        x += text_width(&chamber_text);
    }
    
    // MCU temperatures (if available)
    for mcu in &app.printer.temperatures.mcus {
        let mcu_text = format!("  μC:{:.1}°", mcu.temperature);
        x += text_width(&mcu_text);
    }
    
    // Fan speeds (if available) - make them clickable
    for (fan_idx, fan) in app.printer.temperatures.fans.iter().enumerate() {
        x += text_width("  "); // spacing before fan
        let fan_start = x;
        
        let editing_fan = app.fan_edit_target
            .map(|target| target.index == fan_idx)
            .unwrap_or(false) && app.fan_input.mode == crate::tui::app::InputMode::Editing;
        
        let fan_text = if editing_fan {
            format!("🌀[{}%]", 
                if app.fan_input.value.is_empty() { "_" } else { &app.fan_input.value }
            )
        } else {
            let percent = (fan.speed * 100.0) as u8;
            if let Some(rpm) = fan.rpm {
                format!("🌀{}%({:.0}rpm)", percent, rpm)
            } else {
                format!("🌀{}%", percent)
            }
        };
        
        let fan_width = text_width(&fan_text);
        x += fan_width;
        
        bounds.push((
            TempBarElement::Fan(fan_idx),
            Rect {
                x: fan_start,
                y,
                width: fan_width,
                height: 1,
            },
        ));
    }
    
    bounds
}
