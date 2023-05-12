use crossterm::event::{KeyCode, KeyEvent};

use crate::app::{App, AppResult, DisplayMode, InputMode};

pub async fn handle_key_events(key_event: KeyEvent, app: &mut App) -> AppResult<()> {
    // match key_event.code {
    //     // Quit app
    //     KeyCode::Esc => app.quit(),
    //     // Show help
    //     KeyCode::Char('?') => app.show_help(),
    //     // Show profile selection
    //     KeyCode::Char('p') => app.toggle_profile_selection(),
    //     _ => {}
    // }

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
            KeyCode::Char('p') => app.toggle_profile_selection(),
            _ => {}
        },
        // _ => {}
    }

    match app.display_mode {
        DisplayMode::Empty => {}
        DisplayMode::ProfileSelection => match key_event.code {
            KeyCode::Enter => {
                app.set_profile().await;
                app.set_log_groups().await;
            }
            KeyCode::Esc => app.toggle_profile_selection(),
            KeyCode::Down => app.lists_state.profile_list.next(),
            KeyCode::Up => app.lists_state.profile_list.previous(),
            _ => {}
        },
        DisplayMode::Groups => match key_event.code {
            KeyCode::Enter => {
                app.set_log_streams().await;
            }
            KeyCode::Down => app.lists_state.groups_list.next(),
            KeyCode::Up => app.lists_state.groups_list.previous(),
            _ => {}
        },
        DisplayMode::Streams => match key_event.code {
            KeyCode::Enter => {
                app.set_log_events().await;
            }
            KeyCode::Down => app.lists_state.streams_list.next(),
            KeyCode::Up => app.lists_state.streams_list.previous(),
            _ => {}
        },
        DisplayMode::Events => match key_event.code {
            KeyCode::Down => app.lists_state.events_list.next(),
            KeyCode::Up => app.lists_state.events_list.previous(),
            _ => {}
        },
    }

    Ok(())
}
