use ratatui::Frame;
use ratatui::{
    backend::Backend,
    style::{Color, Modifier, Style},
    text::Spans,
    widgets::{Block, Borders, List, ListItem},
};

use crate::app::App;
use crate::aws::LogSet;
// use crate::profile_set::Profile;
use crate::widgets::popup;
// use crate::widgets::stateful_list::StatefulList;

pub fn render<B: Backend>(frame: &mut Frame<'_, B>, app: &mut App) {
    // app.profile_list = create_stateful_list(&app.profile_set.profiles);

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

// fn create_stateful_list(list_data: &Vec<Profile>) -> StatefulList<Profile> {
//     let mut stateful_list = StatefulList::with_items(list_data.to_vec());
//     stateful_list.state.select(Some(0));
//
//     stateful_list
// }

pub async fn select_profile(app: &mut App) {
    let selected = app.profile_list.select();
    let profile_name = app.profile_list.items[selected].name.to_string();

    app.profile_state.old = app.profile_state.current.to_string();
    app.profile_state.current = profile_name.to_owned();
    app.show_profile_selection();

    let log_set = LogSet::new(profile_name.as_str()).await;
    app.log_set = log_set;
    app.log_set.get_log_groups().await
}
