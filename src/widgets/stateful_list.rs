use ratatui::widgets::ListState;

pub struct StatefulList<T> {
    pub items: Vec<T>,
    pub state: ListState,
}

impl<T> Default for StatefulList<T> {
    fn default() -> Self {
        Self {
            items: vec![],
            state: ListState::default(),
        }
    }
}

impl<T> StatefulList<T> {
    pub fn with_items(items: Vec<T>) -> StatefulList<T> {
        StatefulList {
            state: ListState::default(),
            items,
        }
    }

    pub fn next(&mut self) {
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

    pub fn select(&mut self) -> usize {
        let selected = match self.state.selected() {
            Some(selected) => selected,
            None => 0,
        };

        selected
    }

    pub fn unselect(&mut self) {
        self.state.select(None);
    }
}
