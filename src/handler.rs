use crossterm::event::{KeyCode, KeyEvent};

use crate::app::{App, AppResult, InputMode};

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
            KeyCode::Enter => {
                app.set_profile().await;
                app.set_log_groups().await;
            }
            KeyCode::Esc => app.show_profile_selection(),
            KeyCode::Down => app.lists_state.profile_list.next(),
            // KeyCode::Left => app.profile_list.unselect(),
            KeyCode::Up => app.lists_state.profile_list.previous(),
            _ => {}
        },
    }

    Ok(())
}
