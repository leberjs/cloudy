use ratatui::{
    backend::Backend,
    layout::{Alignment, Constraint, Direction, Layout, Rect},
    style::{Color, Modifier, Style},
    terminal::Frame,
    text::{Span, Spans},
    widgets::{Block, Borders, Clear, Paragraph, Wrap},
};

use crate::app::App;

pub struct Renderer;

impl Renderer {
    pub fn launch<B: Backend>(app: &mut App, frame: &mut Frame<'_, B>) {
        let size = frame.size();

        let chunks = Layout::default()
            .constraints([Constraint::Percentage(10), Constraint::Percentage(80)].as_ref())
            .split(size);

        let main_block = MainBlock::render(&app);
        frame.render_widget(main_block.paragraph, chunks[1]);

        if app.is_showing_profile_selection() {
            let profile_selection = ProfileSelection::render();
            let area = centered_rect(60, 20, size);
            frame.render_widget(Clear, area);
            frame.render_widget(profile_selection.paragraph, area);
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

struct ProfileSelection<'a> {
    paragraph: Paragraph<'a>,
}

impl<'a> ProfileSelection<'a> {
    fn render() -> Self {
        let text = vec![
            Spans::from(""),
            Spans::from(""),
            Spans::from("This is the profile selection area"),
        ];

        let block = Block::default()
            .title("Profile Selection")
            .borders(Borders::ALL)
            .style(Style::default());

        let paragraph = Paragraph::new(text)
            .block(block)
            .alignment(Alignment::Center)
            .wrap(Wrap { trim: true });

        Self { paragraph }
    }
}

// util function for centered rectangle
fn centered_rect(percent_x: u16, percent_y: u16, r: Rect) -> Rect {
    let layout = Layout::default()
        .direction(Direction::Vertical)
        .constraints(
            [
                Constraint::Percentage((100 - percent_y) / 2),
                Constraint::Percentage(percent_y),
                Constraint::Percentage((100 - percent_y) / 2),
            ]
            .as_ref(),
        )
        .split(r);

    Layout::default()
        .direction(Direction::Horizontal)
        .constraints(
            [
                Constraint::Percentage((100 - percent_x) / 2),
                Constraint::Percentage(percent_x),
                Constraint::Percentage((100 - percent_x) / 2),
            ]
            .as_ref(),
        )
        .split(layout[1])[1]
}
