use ratatui::{backend::Backend, Frame};

use crate::app::App;
use crate::renderer::Renderer;

pub fn render<B: Backend>(app: &mut App, frame: &mut Frame<'_, B>) {
    Renderer::launch(app, frame);
}

// TODO: move renderer here
