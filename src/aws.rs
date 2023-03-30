use aws_config::profile::{ProfileFileCredentialsProvider, ProfileFileRegionProvider};
use aws_sdk_cloudwatchlogs::Client;

// use crate::app::AppResult;
use crate::error::{self, ErrorType};

pub struct LogSet {
    pub client: Option<Client>,
    pub events: Vec<()>,
    pub groups: Vec<String>,
    pub streams: Vec<()>,
}

impl Default for LogSet {
    fn default() -> Self {
        Self {
            client: None,
            events: vec![],
            groups: vec![],
            streams: vec![],
        }
    }
}

impl LogSet {
    pub async fn new(profile_name: &str) -> Self {
        let mut log_set = Self::default();

        let region_provider = ProfileFileRegionProvider::builder()
            .profile_name(profile_name)
            .build();

        let config = aws_config::from_env()
            .credentials_provider(
                ProfileFileCredentialsProvider::builder()
                    .profile_name(profile_name)
                    .build(),
            )
            .region(region_provider)
            .load()
            .await;

        log_set.client = Some(Client::new(&config));

        log_set
    }

    pub async fn get_log_groups(&mut self) {
        if let Some(client) = &self.client {
            let resp = client.describe_log_groups().send().await;
            match resp {
                Err(err) => error::handle(err, ErrorType::AWS),
                Ok(r) => {
                    let groups = r.log_groups().unwrap_or_default();
                    for group in groups {
                        self.groups
                            .push(group.log_group_name().unwrap_or_default().to_string());
                    }
                }
            }
        }
    }

    // pub async fn test(app: &mut App) -> Result<(), Error> {
    //     let region_provider = ProfileFileRegionProvider::builder()
    //         .profile_name("leberjs")
    //         .build();
    //
    //     let config = aws_config::from_env()
    //         .credentials_provider(
    //             ProfileFileCredentialsProvider::builder()
    //                 .profile_name("leberjs")
    //                 .build(),
    //         )
    //         .region(region_provider)
    //         .load()
    //         .await;
    //
    //     let client = Client::new(&config);
    //
    //     let resp = client.describe_log_groups().send().await?;
    //     let groups = resp.log_groups().unwrap_or_default();
    //     let mut group_names = vec![];
    //
    //     for group in groups {
    //         group_names.push(group.log_group_name().unwrap_or_default().to_string());
    //     }
    //
    //     app.lg = group_names;
    //
    //     Ok(())
    // }
}
