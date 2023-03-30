use ratatui::Frame;
use ratatui::{
    backend::Backend,
    layout::{Alignment, Rect},
    style::Style,
    text::Spans,
    widgets::{Block, Borders, Paragraph, Wrap},
};

use crate::app::App;

pub enum LogBlockState {
    Empty,
    Populated,
}

pub fn render<B: Backend>(
    frame: &mut Frame<'_, B>,
    app: &App,
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
        LogBlockState::Populated => {
            let text = vec![
                Spans::from(""),
                Spans::from(""),
                Spans::from(""),
                Spans::from(""),
                Spans::from(""),
                if app.is_showing_profile_selection() || app.is_showing_help() {
                    Spans::from("")
                } else {
                    Spans::from("Show groups here")
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
    }
}
