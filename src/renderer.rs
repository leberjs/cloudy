use ratatui::{
    backend::Backend,
    layout::{Alignment, Constraint, Layout},
    style::Style,
    terminal::Frame,
    text::Spans,
    widgets::{Block, Borders, Paragraph, Wrap},
};

use crate::app::App;
use crate::widgets::profile_selection;

pub struct Renderer;

impl Renderer {
    pub fn launch<B: Backend>(app: &App, mut frame: &mut Frame<'_, B>) {
        let size = frame.size();

        let chunks = Layout::default()
            .constraints([Constraint::Percentage(10), Constraint::Percentage(80)].as_ref())
            .split(size);

        let main_block = MainBlock::render(&app);
        frame.render_widget(main_block.paragraph, chunks[1]);

        if app.is_showing_profile_selection() {
            profile_selection::render(&mut frame, &app);
        }
    }
}

struct MainBlock<'a> {
    paragraph: Paragraph<'a>,
}

impl<'a> MainBlock<'a> {
    fn render(app: &App) -> Self {
        let text = vec![
            Spans::from(""),
            Spans::from(""),
            Spans::from(""),
            Spans::from(""),
            Spans::from(""),
            if app.is_showing_profile_selection() {
                Spans::from("Profile selection is showing")
            } else {
                // Spans::from("Press <p> to show Profile selection")
                Spans::from(app.selected_profile.clone())
            },
        ];

        let block = Block::default()
            .title("Logs")
            .borders(Borders::ALL)
            .style(Style::default());

        let paragraph = Paragraph::new(text)
            .block(block)
            .alignment(Alignment::Center)
            .wrap(Wrap { trim: true });

        Self { paragraph }
    }
}
