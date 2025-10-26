use ratatui::widgets::ListState;

/// A stateful list widget for navigable items
/// Inspired by krui's StatefulList pattern
pub struct StatefulList<T> {
    pub state: ListState,
    pub items: Vec<T>,
}

impl<T> StatefulList<T> {
    pub fn new() -> Self {
        StatefulList {
            state: ListState::default(),
            items: Vec::new(),
        }
    }

    pub fn with_items(items: Vec<T>) -> Self {
        let mut list = StatefulList {
            state: ListState::default(),
            items,
        };
        if !list.items.is_empty() {
            list.state.select(Some(0));
        }
        list
    }

    pub fn next(&mut self) {
        if self.items.is_empty() {
            return;
        }
        let i = match self.state.selected() {
            Some(i) => {
                if i >= self.items.len() - 1 {
                    0
                } else {
                    i + 1
                }
            }
            None => 0,
        };
        self.state.select(Some(i));
    }

    pub fn previous(&mut self) {
        if self.items.is_empty() {
            return;
        }
        let i = match self.state.selected() {
            Some(i) => {
                if i == 0 {
                    self.items.len() - 1
                } else {
                    i - 1
                }
            }
            None => 0,
        };
        self.state.select(Some(i));
    }

    pub fn select(&mut self, index: Option<usize>) {
        self.state.select(index);
    }

    pub fn selected(&self) -> Option<&T> {
        self.state.selected().and_then(|i| self.items.get(i))
    }

    pub fn unselect(&mut self) {
        self.state.select(None);
    }
}

impl<T> Default for StatefulList<T> {
    fn default() -> Self {
        Self::new()
    }
}
