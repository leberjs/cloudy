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
        }
        DisplayMode::Streams => {}
        DisplayMode::Events => {}
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
