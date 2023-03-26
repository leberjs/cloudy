use std::io;

use crossterm::event::{DisableMouseCapture, EnableMouseCapture};
use crossterm::terminal::{
    disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen,
};
use ratatui::{backend::Backend, Terminal};

use crate::app::AppResult;

pub struct Ui<B: Backend> {
    terminal: Terminal<B>,
}

impl<B: Backend> Ui<B> {
    pub fn new(terminal: Terminal<B>) -> Self {
        Self { terminal }
    }

    pub fn init(&mut self) -> AppResult<()> {
        enable_raw_mode()?;
        let mut stdout = io::stdout();
        crossterm::execute!(stdout, EnterAlternateScreen, EnableMouseCapture)?;
        self.terminal.hide_cursor()?;
        self.terminal.clear()?;

        Ok(())
    }

    pub fn draw(&mut self) -> AppResult<()> {
        self.terminal.draw(|f| {
            let size = f.size();
            let block = ratatui::widgets::Block::default()
                .title("Block")
                .borders(ratatui::widgets::Borders::ALL);
            f.render_widget(block, size);
        })?;

        Ok(())
    }

    pub fn quit(&mut self) -> AppResult<()> {
        disable_raw_mode()?;
        crossterm::execute!(io::stdout(), LeaveAlternateScreen, DisableMouseCapture)?;
        self.terminal.show_cursor()?;

        Ok(())
    }
}
