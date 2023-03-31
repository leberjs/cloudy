use ratatui::{
    backend::Backend,
    layout::{Alignment, Constraint, Layout},
    style::{Modifier, Style},
    terminal::Frame,
    text::{Span, Spans},
    widgets::{Block, Borders, Paragraph},
};

use crate::app::App;
use crate::widgets::{
    help,
    log_block::{self, LogBlockState},
    profile_selection,
};

pub struct Renderer;

impl Renderer {
    pub fn launch<B: Backend>(app: &mut App, mut frame: &mut Frame<'_, B>) {
        let mut im: &str = "not normal";
        if app.input_mode() == crate::app::InputMode::Normal {
            im = "Normal";
        }

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
            Span::raw(" - Input Mode: "),
            Span::raw(im),
        ]))
        .block(info_block)
        .alignment(Alignment::Left);
        frame.render_widget(info_paragraph, chunks[0]);

        // Conditionally render help popup
        if app.is_showing_help() {
            help::render(&mut frame, &app);
        }

        // Conditionally render profile select popup
        if app.is_showing_profile_selection() {
            profile_selection::render(&mut frame, app);
        }

        // Conditionally render main block content
        if app.log_set.groups.len() == 0 {
            log_block::render(&mut frame, app, LogBlockState::Empty, chunks[2]);
        } else {
            log_block::render(&mut frame, app, LogBlockState::Groups, chunks[2]);
        }
    }
}
