use crate::aws::{
    cloudwatch_logs::{get_log_events, get_log_groups, get_log_streams},
    config::{Client, ProfileSet},
};
use crate::states::{AWSConfigState, AppState, CloudwatchLogState, ListsState};
use crate::widgets::utils::{create_stateful_list, StatefulList};

pub type AppResult<T> = std::result::Result<T, anyhow::Error>;

pub enum DisplayMode {
    Empty,
    Events,
    Groups,
    Streams,
}

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

pub struct App {
    // state
    pub state: AppState,
    pub aws_config_state: AWSConfigState,
    pub lists_state: ListsState,
    pub cloudwatch_log_state: CloudwatchLogState,

    // modes
    pub display_mode: DisplayMode,
    pub help_mode: HelpMode,
    pub input_mode: InputMode,

    pub current_log_display: StatefulList<String>,
}

impl Default for App {
    fn default() -> Self {
        Self {
            // state
            state: AppState::default(),
            aws_config_state: AWSConfigState::default(),
            lists_state: ListsState::default(),
            cloudwatch_log_state: CloudwatchLogState::default(),

            // modes
            display_mode: DisplayMode::Empty,
            help_mode: HelpMode::Normal,
            input_mode: InputMode::Normal,

            current_log_display: StatefulList::default(),
        }
    }
}

impl App {
    pub fn new() -> Self {
        let mut app = Self::default();

        let profile_set = match ProfileSet::load() {
            Ok(p) => p,
            Err(err) => {
                eprintln!("[Error] Issue loading profiles {}", err);
                std::process::exit(1)
            }
        };

        app.aws_config_state.profile_set = profile_set;

        app
    }

    // Initialize app
    pub fn init(&mut self) {
        self.lists_state.profile_list =
            StatefulList::with_items(self.aws_config_state.profile_set.profiles.to_owned());
        self.lists_state.profile_list.state.select(Some(0));
        self.state.is_running = true;
    }

    // Fetch state `show_profile_selection`
    pub fn is_showing_profile_selection(&self) -> bool {
        self.state.show_profile_selection
    }

    // Quit app
    pub fn quit(&mut self) {
        self.state.is_running = false;
    }

    // Set cloudwatch log events
    // pub async fn set_log_events(&mut self) {
    //     self.cloudwatch_log_state.events =
    //         get_log_events(self.aws_config_state.client.clone().unwrap())
    //             .await
    //             .unwrap();
    // }

    // Set cloudwatch log groups
    pub async fn set_log_groups(&mut self) {
        self.cloudwatch_log_state.groups =
            get_log_groups(self.aws_config_state.client.clone().unwrap())
                .await
                .unwrap();

        self.current_log_display = create_stateful_list(&self.cloudwatch_log_state.groups);
        self.display_mode = DisplayMode::Groups;
    }

    // Set cloudwatch log streams
    // pub async fn set_log_events(&mut self) {
    //     self.cloudwatch_log_state.streams =
    //         get_log_events(self.aws_config_state.client.clone().unwrap())
    //             .await
    //             .unwrap();
    // }

    // Set AWS profile
    pub async fn set_profile(&mut self) {
        let selected = self.lists_state.profile_list.select();
        let profile_name = self.lists_state.profile_list.items[selected]
            .name
            .to_owned();

        self.aws_config_state.previous_profile = self.aws_config_state.selected_profile.to_owned();
        self.aws_config_state.selected_profile = profile_name;

        // create client
        self.aws_config_state.client =
            Client::new(self.aws_config_state.selected_profile.as_str()).await;

        self.show_profile_selection();
    }

    // Set Input Mode
    pub fn set_input_mode(&mut self, mode: InputMode) {
        self.input_mode = mode
    }

    // Set state of `show_help`
    pub fn show_help(&mut self) {
        self.state.show_help = !self.state.show_help;
        if self.input_mode == InputMode::Normal {
            self.input_mode = InputMode::Help
        } else {
            self.input_mode = InputMode::Normal
        }
    }

    // Set state of `show_profile_selection` and set Input Mode accordingly
    pub fn show_profile_selection(&mut self) {
        self.state.show_profile_selection = !self.state.show_profile_selection;
        if self.input_mode == InputMode::Normal {
            self.input_mode = InputMode::ProfileSelection
        } else {
            self.input_mode = InputMode::Normal
        }
    }

    pub fn tick(&self) {}
}
