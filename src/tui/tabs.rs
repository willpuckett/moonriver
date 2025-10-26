/// Available tabs in the TUI
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Tab {
    /// GCode console/REPL (press 'c')
    Console,
    /// Toolhead position and homing controls (press 'p')
    Position,
    /// Full job history browser (press 'j')
    Jobs,
    /// Context-sensitive help (press 'h' or '?')
    Help,
}

impl Tab {
    /// Get the key binding for this tab
    #[allow(dead_code)]
    pub fn key(&self) -> char {
        match self {
            Tab::Console => 'c',
            Tab::Position => 'p',
            Tab::Jobs => 'j',
            Tab::Help => 'h',
        }
    }

    /// Get the display name for this tab
    pub fn name(&self) -> &str {
        match self {
            Tab::Console => "Console",
            Tab::Position => "Position",
            Tab::Jobs => "Jobs",
            Tab::Help => "Help",
        }
    }

    /// Try to get a tab from a key press
    pub fn from_key(key: char) -> Option<Self> {
        match key {
            'c' => Some(Tab::Console),
            'p' => Some(Tab::Position),
            'j' => Some(Tab::Jobs),
            'h' | '?' => Some(Tab::Help),
            _ => None,
        }
    }

    /// Get all tabs in order
    #[allow(dead_code)]
    pub fn all() -> &'static [Tab] {
        &[Tab::Console, Tab::Position, Tab::Jobs, Tab::Help]
    }
}

impl Default for Tab {
    fn default() -> Self {
        Tab::Console
    }
}
