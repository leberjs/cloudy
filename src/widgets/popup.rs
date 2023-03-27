use ratatui::{
    backend::Backend,
    layout::{Constraint, Direction, Layout, Rect},
    terminal::Frame,
    widgets::{Clear, StatefulWidget, Widget},
};

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
