pub struct AppState {
    pub is_running: bool,
    pub show_profile_selection: bool,
}

impl Default for AppState {
    fn default() -> Self {
        Self {
            is_running: false,
            show_profile_selection: false,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_app_state() {
        let app_state = AppState::default();
        assert_eq!(app_state.is_running, false);
        assert_eq!(app_state.show_profile_selection, false);
    }
}
