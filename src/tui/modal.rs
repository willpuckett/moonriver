/// Placeholder for modal dialog system
/// Will be implemented when needed for temperature input, print confirmation, etc.

use ratatui::{
    layout::{Alignment, Constraint, Direction, Layout, Rect},
    style::{Color, Modifier, Style},
    text::{Line, Span},
    widgets::{Block, Borders, Clear, Paragraph},
    Frame,
};

pub struct Modal {
    pub title: String,
    pub content: Vec<String>,
}

impl Modal {
    pub fn new(title: impl Into<String>) -> Self {
        Modal {
            title: title.into(),
            content: Vec::new(),
        }
    }

    pub fn with_content(mut self, lines: Vec<String>) -> Self {
        self.content = lines;
        self
    }

    pub fn render(&self, frame: &mut Frame, area: Rect) {
        let block = Block::default()
            .title(self.title.clone())
            .title_alignment(Alignment::Center)
            .borders(Borders::ALL)
            .border_style(Style::default().fg(Color::Cyan))
            .style(Style::default().bg(Color::Black));

        // Center the modal
        let modal_area = centered_rect(60, 50, area);
        
        frame.render_widget(Clear, modal_area);
        frame.render_widget(block, modal_area);

        // Render content inside modal
        let inner = Layout::default()
            .direction(Direction::Vertical)
            .margin(1)
            .split(modal_area);

        let text: Vec<Line> = self.content.iter()
            .map(|s| Line::from(s.clone()))
            .collect();

        let paragraph = Paragraph::new(text)
            .alignment(Alignment::Left);
        
        frame.render_widget(paragraph, inner[0]);
    }
}

/// Helper function to create a centered rectangle
fn centered_rect(percent_x: u16, percent_y: u16, r: Rect) -> Rect {
    let popup_layout = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Percentage((100 - percent_y) / 2),
            Constraint::Percentage(percent_y),
            Constraint::Percentage((100 - percent_y) / 2),
        ])
        .split(r);

    Layout::default()
        .direction(Direction::Horizontal)
        .constraints([
            Constraint::Percentage((100 - percent_x) / 2),
            Constraint::Percentage(percent_x),
            Constraint::Percentage((100 - percent_x) / 2),
        ])
        .split(popup_layout[1])[1]
}
