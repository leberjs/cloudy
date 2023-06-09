use ratatui::Frame;
use ratatui::{
    backend::Backend,
    style::{Color, Style},
    text::{Span, Spans},
    widgets::{Block, Borders, Paragraph},
};

use crate::app::App;
use crate::widgets::utils;

pub fn render<B: Backend>(frame: &mut Frame<'_, B>, _app: &App) {
    let universal = vec![
        Spans::from(Span::styled(
            "?          - toggles help",
            Style::default().fg(Color::Green),
        )),
        Spans::from(Span::styled(
            "Esc        - close menu/exit app",
            Style::default().fg(Color::Red),
        )),
    ];

    let paragraph =
        Paragraph::new(universal).block(Block::default().borders(Borders::ALL).title("Help"));

    utils::render_popup(frame, paragraph, (30, 50))
}
