use ratatui::Frame;
use ratatui::{
    backend::Backend,
    style::{Color, Modifier, Style},
    text::Spans,
    widgets::{Block, Borders, List, ListItem},
};

use crate::app::App;
use crate::widgets::popup;

pub fn render<B: Backend>(frame: &mut Frame<'_, B>, app: &App) {
    let items: Vec<ListItem> = app
        .profile_list
        .items
        .iter()
        .map(|i| {
            let lines = vec![Spans::from(i.name.as_str()), Spans::from("")];
            ListItem::new(lines).style(Style::default())
        })
        .collect();

    let profile_list = List::new(items)
        .block(
            Block::default()
                .borders(Borders::ALL)
                .title("Profile Selection"),
        )
        .highlight_style(
            Style::default()
                .fg(Color::Black)
                .bg(Color::LightBlue)
                .add_modifier(Modifier::BOLD),
        )
        .highlight_symbol(">> ");

    popup::render_popup_with_state(
        frame,
        profile_list,
        app.profile_list.state.clone(),
        (20, 40),
    )
}

pub fn select_profile(app: &mut App) {
    let selected = app.profile_list.select();
    app.profile_state.old = app.profile_state.current.clone();
    app.profile_state.current = app.profile_list.items[selected].name.clone();
    app.show_profile_selection()
}
