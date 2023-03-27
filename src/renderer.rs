use ratatui::{
    backend::Backend,
    layout::{Alignment, Constraint, Layout},
    style::{Modifier, Style},
    terminal::Frame,
    text::{Span, Spans},
    widgets::{Block, Borders, Paragraph, Wrap},
};

use crate::app::App;
use crate::widgets::profile_selection;

pub struct Renderer;

impl Renderer {
    pub fn launch<B: Backend>(app: &App, mut frame: &mut Frame<'_, B>) {
        let size = frame.size();

        let chunks = Layout::default()
            .constraints(
                [
                    Constraint::Percentage(10),
                    Constraint::Percentage(10),
                    Constraint::Percentage(80),
                ]
                .as_ref(),
            )
            .split(size);

        let info_block = Block::default()
            .title("Info (<?> for list of keybinds)")
            .borders(Borders::ALL)
            .style(Style::default());

        let info_paragraph = Paragraph::new(Spans::from(vec![
            Span::raw("Selected Profile: "),
            Span::styled(
                app.profile_state.current.clone(),
                Style::default().add_modifier(Modifier::BOLD),
            ),
            Span::raw("  (Old: "),
            Span::raw(app.profile_state.old.clone()),
            Span::raw(")"),
        ]))
        .block(info_block)
        .alignment(Alignment::Left);
        frame.render_widget(info_paragraph, chunks[0]);

        let main_block = MainBlock::render(&app);
        frame.render_widget(main_block.paragraph, chunks[2]);

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
                Spans::from("")
                // Spans::from("Profile selection is showing")
            } else {
                Spans::from("Press <p> to show Profile selection")
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
