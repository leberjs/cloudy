pub struct AppState {
    pub is_running: bool,
    pub show_help: bool,
    pub show_profile_selection: bool,
}

impl Default for AppState {
    fn default() -> Self {
        Self {
            is_running: false,
            show_help: false,
            show_profile_selection: false,
        }
    }
}

pub struct ProfileState<'a> {
    pub current: &'a str,
    pub old: &'a str,
}

impl<'a> Default for ProfileState<'a> {
    fn default() -> Self {
        Self {
            current: "",
            old: "",
        }
    }
}

pub struct LogState<'a> {
    pub groups: Vec<&'a str>,
    pub streams: Vec<&'a str>,
}

impl<'a> Default for LogState<'a> {
    fn default() -> Self {
        Self {
            groups: Vec::new(),
            streams: Vec::new(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_default_app_state() {
        let app_state = AppState::default();
        assert_eq!(app_state.is_running, false);
        assert_eq!(app_state.show_help, false);
        assert_eq!(app_state.show_profile_selection, false);
    }

    #[test]
    fn test_default_profile_state() {
        let profile_state = ProfileState::default();
        assert_eq!(profile_state.current, "");
        assert_eq!(profile_state.old, "");
    }

    #[test]
    fn test_default_log_state() {
        let log_state = LogState::default();
        assert_eq!(log_state.groups.len(), 0);
        assert_eq!(log_state.streams.len(), 0);
    }
}
