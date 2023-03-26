use crate::profile_set::ProfileSet;
use crate::states::AppState;

pub type AppResult<T> = std::result::Result<T, anyhow::Error>;

pub struct App {
    pub profile_set: ProfileSet,
    pub state: AppState,
}

impl Default for App {
    fn default() -> Self {
        let profile_set = match ProfileSet::load() {
            Ok(p) => p,
            Err(err) => {
                eprintln!("[Error] Issue loading profiles {}", err);
                std::process::exit(1)
            }
        };

        Self {
            profile_set,
            state: AppState::default(),
        }
    }
}

impl App {
    pub fn new() -> Self {
        let app = Self::default();

        app
    }

    // Initialize app
    pub fn init(&mut self) {
        self.state.is_running = true;
    }

    pub fn is_running(&self) -> bool {
        self.state.is_running
    }

    pub fn is_showing_profile_selection(&self) -> bool {
        self.state.show_profile_selection
    }

    // Quit app
    pub fn quit(&mut self) {
        self.state.is_running = false;
    }

    pub fn show_profile_selection(&mut self) {
        self.state.show_profile_selection = !self.state.show_profile_selection
    }
}
