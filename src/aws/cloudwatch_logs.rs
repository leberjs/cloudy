use aws_sdk_cloudwatchlogs::Client as AWSClient;

use crate::error::{self, ErrorType};

pub async fn get_log_groups(client: Option<AWSClient>) -> Result<Vec<String>, ()> {
    if let Some(client) = client {
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
    } else {
        Ok(vec![])
    }
}
