use std::io;

use crossterm::event::DisableMouseCapture;
use crossterm::terminal::{disable_raw_mode, LeaveAlternateScreen};

use crate::app::AppResult;

pub enum ErrorType {
    AWS,
    Custom,
}

pub fn handle<E>(err: E, error_type: ErrorType)
where
    E: std::fmt::Debug,
{
    match error_type {
        ErrorType::AWS => {
            bail().expect("ERROR ERROR ERROR");
            eprintln!("[AWS Error] {:#?}", err);
            std::process::exit(1)
        }
        ErrorType::Custom => {
            bail().expect("ERROR ERROR ERROR");
            eprintln!("[Error] {:#?}", err);
            std::process::exit(1)
        }
    }
}

fn bail() -> AppResult<()> {
    disable_raw_mode()?;
    crossterm::execute!(io::stdout(), LeaveAlternateScreen, DisableMouseCapture)?;

    Ok(())
}
