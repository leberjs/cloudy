use aws_sdk_cloudwatchlogs::Client as AWSClient;

use crate::aws::config::{Profile, ProfileSet};
use crate::widgets::utils::StatefulList;

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

pub struct AWSConfigState {
    pub client: Option<AWSClient>,
    pub profile_set: ProfileSet,
    pub selected_profile: String,
    pub previous_profile: String,
}

impl Default for AWSConfigState {
    fn default() -> Self {
        Self {
            client: None,
            profile_set: ProfileSet {
                profiles: Vec::new(),
            },
            selected_profile: String::from(""),
            previous_profile: String::from(""),
        }
    }
}

pub struct ListsState {
    pub groups_list: StatefulList<String>,
    pub profile_list: StatefulList<Profile>,
}

impl Default for ListsState {
    fn default() -> Self {
        Self {
            profile_list: StatefulList::default(),
            groups_list: StatefulList::default(),
        }
    }
}

pub struct CloudwatchLogState {
    pub groups: Vec<String>,
    pub streams: Vec<String>,
    pub events: Vec<String>,
    pub selected_log_group: String,
    pub selected_log_stream: String,
}

impl Default for CloudwatchLogState {
    fn default() -> Self {
        Self {
            groups: Vec::new(),
            streams: Vec::new(),
            events: Vec::new(),
            selected_log_group: String::from(""),
            selected_log_stream: String::from(""),
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
    fn test_default_aws_config_state() {
        let aws_config_state = AWSConfigState::default();
        assert!(aws_config_state.client.is_none());
        assert_eq!(aws_config_state.profile_set.profiles.len(), 0);
        assert_eq!(aws_config_state.selected_profile, "");
        assert_eq!(aws_config_state.previous_profile, "");
    }

    #[test]
    fn test_default_lists_state() {
        let lists_state = ListsState::default();
        assert_eq!(lists_state.groups_list.items.len(), 0);
        assert_eq!(lists_state.profile_list.items.len(), 0);
    }

    #[test]
    fn test_default_log_state() {
        let cloudwatch_log_state = CloudwatchLogState::default();
        assert_eq!(cloudwatch_log_state.groups.len(), 0);
        assert_eq!(cloudwatch_log_state.streams.len(), 0);
        assert_eq!(cloudwatch_log_state.events.len(), 0);
        assert_eq!(cloudwatch_log_state.selected_log_group, "");
        assert_eq!(cloudwatch_log_state.selected_log_stream, "");
    }
}
