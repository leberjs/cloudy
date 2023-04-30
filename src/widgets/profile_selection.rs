use ratatui::Frame;
use ratatui::{
    backend::Backend,
    style::{Color, Modifier, Style},
    text::Spans,
    widgets::{Block, Borders, List, ListItem},
};

use crate::app::App;
use crate::aws::cloudwatch_logs::LogSet;
use crate::widgets::log_block::create_stateful_list;
use crate::widgets::popup;

pub fn render<B: Backend>(frame: &mut Frame<'_, B>, app: &mut App) {
    let items: Vec<ListItem> = app
        .profile_list
        .items
        .iter()
        .map(|i| {
            let lines = vec![Spans::from(i.name.to_owned()), Spans::from("")];
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

pub async fn select_profile(app: &mut App) {
    let selected = app.profile_list.select();
    let profile_name = app.profile_list.items[selected].name.clone();

    app.profile_state.old = app.profile_state.current.to_owned();
    app.profile_state.current = profile_name.clone();
    app.show_profile_selection();

    let log_set = LogSet::new(profile_name.as_str()).await;
    app.log_set = log_set;
    app.log_set.get_log_groups().await;

    app.current_log_display = create_stateful_list(&app.log_set.groups);
}
