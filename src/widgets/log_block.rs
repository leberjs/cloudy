use ratatui::Frame;
use ratatui::{
    backend::Backend,
    layout::{Alignment, Rect},
    style::{Color, Modifier, Style},
    text::Spans,
    widgets::{Block, Borders, List, ListItem, Paragraph, Wrap},
};

use crate::app::App;
use crate::widgets::stateful_list::StatefulList;

pub enum LogBlockState {
    Empty,
    Groups,
}

pub fn render<B: Backend>(
    frame: &mut Frame<'_, B>,
    app: &mut App,
    log_block_state: LogBlockState,
    area: Rect,
) {
    match log_block_state {
        LogBlockState::Empty => {
            let text = vec![
                Spans::from(""),
                Spans::from(""),
                Spans::from(""),
                Spans::from(""),
                Spans::from(""),
                if app.is_showing_profile_selection() || app.is_showing_help() {
                    Spans::from("")
                } else {
                    Spans::from("Press <p> to show Profile selection")
                },
            ];

            let paragraph = Paragraph::new(text)
                .block(
                    Block::default()
                        .title("Logs")
                        .borders(Borders::ALL)
                        .style(Style::default()),
                )
                .alignment(Alignment::Center)
                .wrap(Wrap { trim: true });

            frame.render_widget(paragraph, area)
        }
        LogBlockState::Groups => {
            // app.current_log_display = create_stateful_list(&app.log_set.groups);

            let items: Vec<ListItem> = app
                .current_log_display
                .items
                .iter()
                .map(|i| {
                    let lines = vec![Spans::from(i.as_str()), Spans::from("")];
                    ListItem::new(lines).style(Style::default())
                })
                .collect();

            let log_group_list = List::new(items)
                .block(Block::default().borders(Borders::ALL).title("Log Groups"))
                .highlight_style(
                    Style::default()
                        .fg(Color::Black)
                        .bg(Color::LightBlue)
                        .add_modifier(Modifier::BOLD),
                )
                .highlight_symbol(">> ");

            let mut state = app.current_log_display.state.clone();

            frame.render_stateful_widget(log_group_list, area, &mut state)
            // frame.render_stateful_widget(log_group_list, area, &mut app.current_log_display.state)
        }
    }
}

pub fn create_stateful_list(list_data: &Vec<String>) -> StatefulList<String> {
    let mut stateful_list = StatefulList::with_items(list_data.clone());
    // let mut stateful_list = StatefulList::with_items(list_data.to_vec());
    stateful_list.state.select(Some(0));

    stateful_list
}
