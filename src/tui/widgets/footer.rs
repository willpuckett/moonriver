use crate::tui::app::App;
use crate::tui::tabs::Tab;
use ratatui::{
    layout::Rect,
    style::{Color, Modifier, Style},
    text::{Line, Span},
    widgets::Paragraph,
    Frame,
};

/// Clickable actions in the footer
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum FooterAction {
    Tab(Tab),
    Escape,
    Quit,
}

pub fn render(frame: &mut Frame, area: Rect, app: &App) {
    let mut spans = vec![];

    // Create consistent tab bar with highlighting for active tab
    // Console tab
    if app.current_tab == Tab::Console {
        spans.push(Span::styled(
            " [C]onsole ",
            Style::default()
                .fg(Color::Black)
                .bg(Color::Cyan)
                .add_modifier(Modifier::BOLD),
        ));
    } else {
        spans.push(Span::styled(
            " [C]",
            Style::default().fg(Color::Cyan),
        ));
        spans.push(Span::styled(
            "onsole ",
            Style::default().fg(Color::White),
        ));
    }

    spans.push(Span::raw("│ "));

    // Position tab
    if app.current_tab == Tab::Position {
        spans.push(Span::styled(
            " [P]osition ",
            Style::default()
                .fg(Color::Black)
                .bg(Color::Cyan)
                .add_modifier(Modifier::BOLD),
        ));
    } else {
        spans.push(Span::styled(
            " [P]",
            Style::default().fg(Color::Cyan),
        ));
        spans.push(Span::styled(
            "osition ",
            Style::default().fg(Color::White),
        ));
    }

    spans.push(Span::raw("│ "));

    // Jobs tab
    if app.current_tab == Tab::Jobs {
        spans.push(Span::styled(
            " [J]obs ",
            Style::default()
                .fg(Color::Black)
                .bg(Color::Cyan)
                .add_modifier(Modifier::BOLD),
        ));
    } else {
        spans.push(Span::styled(
            " [J]",
            Style::default().fg(Color::Cyan),
        ));
        spans.push(Span::styled(
            "obs ",
            Style::default().fg(Color::White),
        ));
    }

    spans.push(Span::raw("│ "));

    // Help tab
    if app.current_tab == Tab::Help {
        spans.push(Span::styled(
            " [H]elp ",
            Style::default()
                .fg(Color::Black)
                .bg(Color::Cyan)
                .add_modifier(Modifier::BOLD),
        ));
    } else {
        spans.push(Span::styled(
            " [H]",
            Style::default().fg(Color::Cyan),
        ));
        spans.push(Span::styled(
            "elp ",
            Style::default().fg(Color::White),
        ));
    }

    // Add some spacing
    spans.push(Span::raw(" │ "));

    // Context-sensitive controls based on current tab
    match app.current_tab {
        Tab::Console => {
            spans.extend(vec![
                Span::styled("[I]", Style::default().fg(Color::Yellow)),
                Span::raw("nput "),
                Span::styled("[↑↓]", Style::default().fg(Color::Yellow)),
                Span::raw("History "),
                Span::styled("[Enter]", Style::default().fg(Color::Green)),
                Span::raw("Send "),
            ]);
        }
        Tab::Position => {
            spans.extend(vec![
                Span::styled("[X/Y/Z]", Style::default().fg(Color::Yellow)),
                Span::raw("Home "),
                Span::styled("[A]", Style::default().fg(Color::Yellow)),
                Span::raw("ll "),
            ]);
        }
        Tab::Jobs => {
            spans.extend(vec![
                Span::styled("[↑↓]", Style::default().fg(Color::Yellow)),
                Span::raw("Nav "),
                Span::styled("[Enter]", Style::default().fg(Color::Green)),
                Span::raw("Start "),
                Span::styled("[R]", Style::default().fg(Color::Yellow)),
                Span::raw("efresh "),
            ]);
        }
        Tab::Help => {
            spans.extend(vec![
                Span::styled("[Esc]", Style::default().fg(Color::Yellow)),
                Span::raw("Back "),
            ]);
        }
    }

    // Global panel toggles (work from all tabs)
    spans.push(Span::raw("│ "));
    spans.extend(vec![
        Span::styled("[T]", Style::default().fg(Color::Magenta)),
        Span::raw("emp "),
        Span::styled("[L]", Style::default().fg(Color::Magenta)),
        Span::raw("oc "),
    ]);

    spans.push(Span::raw("│ "));
    spans.push(Span::styled("[Q]", Style::default().fg(Color::Red)));
    spans.push(Span::raw("uit"));

    let footer = Paragraph::new(Line::from(spans))
        .style(Style::default().bg(Color::DarkGray).fg(Color::White));

    frame.render_widget(footer, area);
}

/// Calculate the bounds for each clickable element in the footer
/// Returns a vector of (FooterAction, Rect) tuples representing clickable areas
pub fn get_footer_bounds(area: Rect, current_tab: Tab) -> Vec<(FooterAction, Rect)> {
    let mut bounds = Vec::new();
    let y = area.y;
    let mut x = area.x;

    // Console: " [C]onsole "
    bounds.push((
        FooterAction::Tab(Tab::Console),
        Rect {
            x,
            y,
            width: 11,
            height: 1,
        },
    ));
    x += 11 + 2;

    // Position: " [P]osition "
    bounds.push((
        FooterAction::Tab(Tab::Position),
        Rect {
            x,
            y,
            width: 12,
            height: 1,
        },
    ));
    x += 12 + 2;

    // Jobs: " [J]obs "
    bounds.push((
        FooterAction::Tab(Tab::Jobs),
        Rect {
            x,
            y,
            width: 8,
            height: 1,
        },
    ));
    x += 8 + 2;

    // Help: " [H]elp "
    bounds.push((
        FooterAction::Tab(Tab::Help),
        Rect {
            x,
            y,
            width: 8,
            height: 1,
        },
    ));
    
    // Add spacing for separator
    x += 10 + 3; // " │ "

    // Context-sensitive controls - calculate their positions
    match current_tab {
        Tab::Console => {
            // "[I]nput [↑↓]History [Enter]Send "
            x += 7 + 11 + 12; // Skip context shortcuts
        }
        Tab::Position => {
            // "[X/Y/Z]Home [A]ll "
            x += 11 + 5; // Skip context shortcuts
        }
        Tab::Jobs => {
            // "[↑↓]Nav [Enter]Start [R]efresh "
            x += 8 + 12 + 13; // Skip context shortcuts
        }
        Tab::Help => {
            // "[Esc]Back "
            // This one is special - make it clickable!
            bounds.push((
                FooterAction::Escape,
                Rect {
                    x,
                    y,
                    width: 10, // "[Esc]Back "
                    height: 1,
                },
            ));
            x += 10;
        }
    }

    // Separator before Quit and global toggles
    x += 2; // "│ "
    // "[T]emp [L]oc │ "
    x += 6 + 5 + 3;

    // Quit: "[Q]uit"
    bounds.push((
        FooterAction::Quit,
        Rect {
            x,
            y,
            width: 6, // "[Q]uit"
            height: 1,
        },
    ));

    bounds
}

/// Calculate the bounds for each tab button (deprecated, use get_footer_bounds instead)
/// Returns a vector of (Tab, Rect) tuples representing clickable areas
#[allow(dead_code)]
pub fn get_tab_bounds(area: Rect) -> Vec<(Tab, Rect)> {
    let mut bounds = Vec::new();
    let y = area.y;
    let mut x = area.x;

    // Console: " [C]onsole "
    bounds.push((
        Tab::Console,
        Rect {
            x,
            y,
            width: 11,
            height: 1,
        },
    ));
    x += 11 + 2;

    // Position: " [P]osition "
    bounds.push((
        Tab::Position,
        Rect {
            x,
            y,
            width: 12,
            height: 1,
        },
    ));
    x += 12 + 2;

    // Jobs: " [J]obs "
    bounds.push((
        Tab::Jobs,
        Rect {
            x,
            y,
            width: 8,
            height: 1,
        },
    ));
    x += 8 + 2;

    // Help: " [H]elp "
    bounds.push((
        Tab::Help,
        Rect {
            x,
            y,
            width: 8,
            height: 1,
        },
    ));

    bounds
}
