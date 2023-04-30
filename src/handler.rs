use crossterm::event::{KeyCode, KeyEvent};

use crate::app::{App, AppResult, InputMode};
use crate::widgets::profile_selection;

pub async fn handle_key_events(key_event: KeyEvent, app: &mut App) -> AppResult<()> {
    match app.input_mode {
        InputMode::Help => match key_event.code {
            KeyCode::Esc => app.show_help(),
            _ => {}
        },
        InputMode::Normal => match key_event.code {
            // Quit app
            KeyCode::Esc => app.quit(),
            // Show help
            KeyCode::Char('?') => app.show_help(),
            // Show profile selection
            KeyCode::Char('p') => app.show_profile_selection(),
            // Log Block navigation
            KeyCode::Down => app.current_log_display.next(),
            KeyCode::Up => app.current_log_display.previous(),
            _ => {}
        },
        InputMode::ProfileSelection => match key_event.code {
            // TODO: move these types of calls to app and delegate from there
            KeyCode::Enter => profile_selection::select_profile(app).await,
            KeyCode::Esc => app.show_profile_selection(),
            KeyCode::Down => app.profile_list.next(),
            // KeyCode::Left => app.profile_list.unselect(),
            KeyCode::Up => app.profile_list.previous(),
            _ => {}
        },
    }

    Ok(())
}
