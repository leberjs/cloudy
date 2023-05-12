use ratatui::Frame;
use ratatui::{
    backend::Backend,
    layout::{Alignment, Rect},
    style::{Color, Modifier, Style},
    text::Spans,
    widgets::{Block, Borders, List, ListItem, Paragraph, Wrap},
};

use crate::app::{App, DisplayMode};

pub fn render<B: Backend>(frame: &mut Frame<'_, B>, app: &mut App, area: Rect) {
    match app.display_mode {
        DisplayMode::Groups => {
            let items: Vec<ListItem> = app
                .lists_state
                .groups_list
                .items
                .iter()
                .map(|i| {
                    let lines = vec![Spans::from(i.as_str()), Spans::from("")];
                    ListItem::new(lines).style(Style::default())
                })
                .collect();

            let title = format!("Log Groups: {}", app.cloudwatch_log_state.groups.len());

            let log_group_list = List::new(items)
                .block(Block::default().borders(Borders::ALL).title(title))
                .highlight_style(
                    Style::default()
                        .fg(Color::Black)
                        .bg(Color::LightBlue)
                        .add_modifier(Modifier::BOLD),
                )
                .highlight_symbol(">> ");

            let mut state = app.lists_state.groups_list.state.clone();

            frame.render_stateful_widget(log_group_list, area, &mut state)
        }
        DisplayMode::Streams => {
            let items: Vec<ListItem> = app
                .lists_state
                .streams_list
                .items
                .iter()
                .map(|i| {
                    let lines = vec![Spans::from(i.as_str()), Spans::from("")];
                    ListItem::new(lines).style(Style::default())
                })
                .collect();

            let title = format!("Log Streams: {}", app.cloudwatch_log_state.streams.len());

            let log_stream_list = List::new(items)
                .block(Block::default().borders(Borders::ALL).title(title))
                .highlight_style(
                    Style::default()
                        .fg(Color::Black)
                        .bg(Color::LightBlue)
                        .add_modifier(Modifier::BOLD),
                )
                .highlight_symbol(">> ");

            let mut state = app.lists_state.streams_list.state.clone();

            frame.render_stateful_widget(log_stream_list, area, &mut state)
        }
        DisplayMode::Events => {
            let items: Vec<ListItem> = app
                .lists_state
                .events_list
                .items
                .iter()
                .map(|i| {
                    let lines = vec![Spans::from(i.as_str()), Spans::from("")];
                    ListItem::new(lines).style(Style::default())
                })
                .collect();

            let title = format!("Log Events: {}", app.cloudwatch_log_state.events.len());

            let log_event_list = List::new(items)
                .block(Block::default().borders(Borders::ALL).title(title))
                .highlight_style(
                    Style::default()
                        .fg(Color::Black)
                        .bg(Color::LightBlue)
                        .add_modifier(Modifier::BOLD),
                )
                .highlight_symbol(">> ");

            let mut state = app.lists_state.events_list.state.clone();

            frame.render_stateful_widget(log_event_list, area, &mut state)
        }
        _ => {
            let paragraph = generate_default_display(&app);
            frame.render_widget(paragraph, area)
        }
    }
}

fn generate_default_display(app: &App) -> Paragraph {
    let text = vec![
        Spans::from(""),
        Spans::from(""),
        Spans::from(""),
        Spans::from(""),
        Spans::from(""),
        if app.state.show_profile_selection || app.state.show_help {
            Spans::from("")
        } else {
            Spans::from("Press <p> to show Profile selection")
        },
    ];

    let paragraph = Paragraph::new(text)
        .block(
            Block::default()
                .borders(Borders::ALL)
                .style(Style::default()),
        )
        .alignment(Alignment::Center)
        .wrap(Wrap { trim: true });

    paragraph
}
