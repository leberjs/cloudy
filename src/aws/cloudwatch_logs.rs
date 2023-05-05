use aws_sdk_cloudwatchlogs::Client as AWSClient;

use crate::error::{self, ErrorType};

pub async fn get_log_groups(client: AWSClient) -> Result<Vec<String>, ()> {
    let resp = client.describe_log_groups().send().await;
    match resp {
        Err(err) => Err(error::handle(err, ErrorType::AWS)),
        Ok(r) => {
            let groups = r.log_groups().unwrap_or_default();
            let mut res = vec![];
            for group in groups {
                res.push(group.log_group_name().unwrap_or_default().to_string());
            }

            Ok(res)
        }
    }
}

pub async fn get_log_streams(client: AWSClient, log_group_name: &str) -> Result<Vec<String>, ()> {
    let resp = client
        .describe_log_streams()
        .log_group_name(log_group_name)
        .send()
        .await;
    match resp {
        Err(err) => Err(error::handle(err, ErrorType::AWS)),
        Ok(r) => {
            let streams = r.log_streams().unwrap_or_default();
            let mut res = vec![];
            for stream in streams {
                res.push(stream.log_stream_name().unwrap_or_default().to_string());
            }

            Ok(res)
        }
    }
}

pub async fn get_log_events(
    client: AWSClient,
    log_group_name: &str,
    log_stream_name: &str,
) -> Result<Vec<String>, ()> {
    let resp = client
        .get_log_events()
        .log_group_name(log_group_name)
        .log_stream_name(log_stream_name)
        .send()
        .await;
    match resp {
        Err(err) => Err(error::handle(err, ErrorType::AWS)),
        Ok(r) => {
            let events = r.events().unwrap_or_default();
            let mut res = vec![];
            for event in events {
                res.push(event.message().unwrap_or_default().to_string());
            }

            Ok(res)
        }
    }
}
