use ratatui::{
    backend::Backend,
    layout::{Constraint, Direction, Layout, Rect},
    terminal::Frame,
    widgets::{Clear, ListState, StatefulWidget, Widget},
};

/// StatefulList
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

/// helper function to create a stateful list
pub fn create_stateful_list(list_data: &Vec<String>) -> StatefulList<String> {
    let mut stateful_list = StatefulList::with_items(list_data.clone());
    stateful_list.state.select(Some(0));

    stateful_list
}

/// Popup
pub fn render_popup<B: Backend, W: Widget>(
    frame: &mut Frame<'_, B>,
    content: W,
    percent_area: (u16, u16),
) {
    let size = frame.size();
    let area = centered_rect(percent_area.0, percent_area.1, size);

    frame.render_widget(Clear, area);
    frame.render_widget(content, area);
}

pub fn render_popup_with_state<B: Backend, SW: StatefulWidget>(
    frame: &mut Frame<'_, B>,
    content: SW,
    mut state: SW::State,
    percent_area: (u16, u16),
) {
    let size = frame.size();
    let area = centered_rect(percent_area.0, percent_area.1, size);

    frame.render_widget(Clear, area);
    frame.render_stateful_widget(content, area, &mut state)
}

/// helper function for centered rect using up certain percentage of the available rect `r`
fn centered_rect(percent_x: u16, percent_y: u16, r: Rect) -> Rect {
    let layout = Layout::default()
        .direction(Direction::Vertical)
        .constraints(
            [
                Constraint::Percentage((100 - percent_y) / 2),
                Constraint::Percentage(percent_y),
                Constraint::Percentage((100 - percent_y) / 2),
            ]
            .as_ref(),
        )
        .split(r);

    Layout::default()
        .direction(Direction::Horizontal)
        .constraints(
            [
                Constraint::Percentage((100 - percent_x) / 2),
                Constraint::Percentage(percent_x),
                Constraint::Percentage((100 - percent_x) / 2),
            ]
            .as_ref(),
        )
        .split(layout[1])[1]
}
