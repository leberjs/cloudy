use ratatui::{
    backend::Backend,
    layout::{Alignment, Constraint, Layout},
    style::{Modifier, Style},
    text::{Span, Spans},
    widgets::{Block, Borders, Paragraph},
    Frame,
};

use crate::widgets::{help, main_block, profile_selection};

use crate::app::App;

pub fn render<B: Backend>(app: &mut App, frame: &mut Frame<'_, B>) {
    Renderer::launch(app, frame);
}

struct Renderer;

// NOTE: Keep setting of state away from here!
impl Renderer {
    fn launch<B: Backend>(app: &mut App, mut frame: &mut Frame<'_, B>) {
        // TODO: remove
        // let mut im: &str = "not normal";
        // if app.input_mode() == crate::app::InputMode::Normal {
        //     im = "Normal";
        // }

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
                app.aws_config_state.selected_profile.clone(),
                Style::default().add_modifier(Modifier::BOLD),
            ),
            Span::raw("  (Old: "),
            Span::raw(app.aws_config_state.previous_profile.clone()),
            Span::raw(")"),
            // Span::raw(" - Input Mode: "), TODO: remove
            // Span::raw(im), TODO: remove
        ]))
        .block(info_block)
        .alignment(Alignment::Left);
        frame.render_widget(info_paragraph, chunks[0]);

        // Conditionally render help popup
        if app.state.show_help {
            help::render(&mut frame, &app);
        }

        // Conditionally render profile select popup
        if app.state.show_profile_selection {
            profile_selection::render(&mut frame, app);
        }

        // Conditionally render main block content
        // match app.display_mode {
        //     DisplayMode::Empty => main_block::render(&mut frame, app, chunks[2]),
        //     DisplayMode::Groups => main_block::render(&mut frame, app, chunks[2]),
        //     DisplayMode::Streams => {}
        //     DisplayMode::Events => {}
        //     _ => {}
        // }
        // Render main block
        main_block::render(&mut frame, app, chunks[2]);
    }
}
