use crate::profile_set::{Profile, ProfileSet};
use crate::states::{AppState, ProfileState};
use crate::widgets::stateful_list::StatefulList;

pub type AppResult<T> = std::result::Result<T, anyhow::Error>;

#[derive(Clone, Copy, PartialEq)]
pub enum InputMode {
    Normal,
    ProfileSelection,
}

pub struct App {
    pub input_mode: InputMode,
    pub profile_list: StatefulList<Profile>,
    pub profile_set: ProfileSet,
    pub profile_state: ProfileState,
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
            input_mode: InputMode::Normal,
            profile_list: StatefulList::default(),
            profile_set,
            profile_state: ProfileState::default(),
            state: AppState::default(),
        }
    }
}

impl App {
    pub fn new() -> Self {
        let mut app = Self::default();

        app.profile_list = StatefulList::with_items(app.profile_set.profiles.clone());

        app
    }

    // Initialize app
    pub fn init(&mut self) {
        self.state.is_running = true;
    }

    // Fetch Input Mode
    pub fn input_mode(&self) -> InputMode {
        self.input_mode
    }

    // Fetch state `is_running`
    pub fn is_running(&self) -> bool {
        self.state.is_running
    }

    // Fetch state `is_showing_profile_selection`
    pub fn is_showing_profile_selection(&self) -> bool {
        self.state.show_profile_selection
    }

    // Quit app
    pub fn quit(&mut self) {
        self.state.is_running = false;
    }

    // Set Input Mode
    pub fn set_input_mode(&mut self, mode: InputMode) {
        self.input_mode = mode
    }

    // Set state of `show_profile_selection` and set Input Mode accordingly
    pub fn show_profile_selection(&mut self) {
        self.state.show_profile_selection = !self.state.show_profile_selection;
        if self.input_mode() == InputMode::Normal {
            self.input_mode = InputMode::ProfileSelection
        } else {
            self.input_mode = InputMode::Normal
        }
    }
}
