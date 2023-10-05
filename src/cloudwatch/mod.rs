use std::sync::Arc;

use aws_config::SdkConfig;
use aws_sdk_cloudwatchlogs::{types::InputLogEvent, Client};
use regex::Regex;
use tokio::runtime::Runtime;


/// CloudWatchClient is a wrapper around the AWS SDK for Rust CloudWatchLogs client
/// It provides a simple interface to create a log group, create a log stream, and put log events
#[derive(Clone)]
pub struct CloudWatchClient {
    cloudwatch_client: Arc<Client>,
    log_group_name: String,
    log_stream_name: String,
    runtime: Arc<Runtime>
}

#[async_trait::async_trait]
pub trait CloudWatchClientTrait {
    async fn create_log_group(&self) -> Result<(), Box<dyn std::error::Error>>;
    async fn create_log_stream(&self) -> Result<(), Box<dyn std::error::Error>>;
    fn put_log_events(&self, log_events: Vec<String>) -> Result<(), Box<dyn std::error::Error>>;
    async fn init(&self) -> Result<(), Box<dyn std::error::Error>> {
        self.create_log_group().await?;
        self.create_log_stream().await?;
        Ok(())
    }
}

impl CloudWatchClient {
    #[allow(dead_code)]
    pub fn new(client: SdkConfig, log_group_name: String, log_stream_name: String) -> Self {
        let cloudwatch_client = Arc::new(aws_sdk_cloudwatchlogs::Client::new(&client));
        let runtime = Arc::new(Runtime::new().unwrap());
        Self {
            cloudwatch_client,
            log_group_name,
            log_stream_name,
            runtime
        }
    }

    fn remove_ansi_codes(&self, input: &str) -> String {
        let re = Regex::new(r"\x1B\[([0-9]{1,2}(;[0-9]{1,2})?)?[mGK]").unwrap();
        re.replace_all(input, "").to_string()
    }
}

#[async_trait::async_trait]
impl CloudWatchClientTrait for CloudWatchClient {
    async fn create_log_group(&self) -> Result<(), Box<dyn std::error::Error>> {
        match self
            .cloudwatch_client
            .create_log_group()
            .set_log_group_name(Some(self.log_group_name.clone()))
            .send()
            .await
        {
            Ok(_) => Ok(()),
            Err(_) => Ok(()),
        }
    }

    async fn create_log_stream(&self) -> Result<(), Box<dyn std::error::Error>> {
        match self
            .cloudwatch_client
            .create_log_stream()
            .set_log_group_name(Some(self.log_group_name.clone()))
            .set_log_stream_name(Some(self.log_stream_name.clone()))
            .send()
            .await
        {
            Ok(_) => Ok(()),
            Err(_) => Ok(()),
        }
    }

    fn put_log_events(&self, log_events: Vec<String>) -> Result<(), Box<dyn std::error::Error>> {
        let mut batch = Vec::new();
        for log_event in log_events {
            println!("{}", log_event);
            let log_event_without_date = log_event
                .split(" ")
                .skip(1)
                .collect::<Vec<&str>>()
                .join(" ");
            let log_event_without_date = &self.remove_ansi_codes(&log_event_without_date);
            let log_event = InputLogEvent::builder()
                .message(format!("{}", log_event_without_date))
                .timestamp(chrono::Utc::now().timestamp_millis())
                .build();

            batch.push(log_event);
        }

        let send_future = self
            .cloudwatch_client
            .put_log_events()
            .set_log_group_name(Some(self.log_group_name.clone()))
            .set_log_stream_name(Some(self.log_stream_name.clone()))
            .set_log_events(Some(batch))
            .send();

        let rt = self.runtime.clone();
        tokio::task::spawn_blocking(move || {
            let _ = rt.block_on(send_future).unwrap();
        });
        Ok(())
    }
}
