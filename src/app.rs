use crate::aws::LogSet;
use crate::profile_set::{Profile, ProfileSet};
use crate::states::{AppState, LogState, ProfileState};
use crate::widgets::stateful_list::StatefulList;

pub type AppResult<T> = std::result::Result<T, anyhow::Error>;

pub enum HelpMode {
    Normal,
    ProfileSelection,
}

#[derive(Clone, Copy, PartialEq)]
pub enum InputMode {
    Help,
    Normal,
    ProfileSelection,
}

pub struct App<'a> {
    // state
    pub state: AppState,
    pub log_state: LogState<'a>,
    pub profile_state: ProfileState<'a>,

    // modes
    pub help_mode: HelpMode,
    pub input_mode: InputMode,

    pub current_log_display: StatefulList<String>,
    pub log_set: LogSet,
    pub profile_list: StatefulList<Profile>,
    pub profile_set: ProfileSet,
}

impl Default for App<'_> {
    fn default() -> Self {
        let profile_set = match ProfileSet::load() {
            Ok(p) => p,
            Err(err) => {
                eprintln!("[Error] Issue loading profiles {}", err);
                std::process::exit(1)
            }
        };

        Self {
            // state
            state: AppState::default(),
            log_state: LogState::default(),
            profile_state: ProfileState::default(),

            // modes
            help_mode: HelpMode::Normal,
            input_mode: InputMode::Normal,

            current_log_display: StatefulList::default(),
            log_set: LogSet::default(),
            profile_list: StatefulList::default(),
            profile_set,
        }
    }
}

impl App<'_> {
    pub fn new() -> Self {
        let mut app = Self::default();

        app.profile_list = StatefulList::with_items(app.profile_set.profiles.clone());
        app.profile_list.state.select(Some(0));

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

    // Fetch state `show_help`
    pub fn is_showing_help(&self) -> bool {
        self.state.show_help
    }

    // Fetch state `show_profile_selection`
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

    // Set state of `show_help`
    pub fn show_help(&mut self) {
        self.state.show_help = !self.state.show_help;
        if self.input_mode() == InputMode::Normal {
            self.input_mode = InputMode::Help
        } else {
            self.input_mode = InputMode::Normal
        }
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
