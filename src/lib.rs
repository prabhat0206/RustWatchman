use std::sync::Arc;

use aws_config::SdkConfig;
use cloudwatch::{CloudWatchClient, CloudWatchClientTrait};

mod cloudwatch;
mod logger;


/// Get the writer to be used with tracing_subscriber
/// ```
/// use rustwatchman::Watchman;
/// 
/// #[tokio::main]
/// async fn main() {
///    let sdk_config = aws_config::load_from_env().await; 
///    let writer = Watchman::new(sdk_config, "test".to_string(), "new".to_string()).await.get_writer();
///    tracing_subscriber::fmt()
///       .with_max_level(tracing::Level::INFO)
///       .with_target(false)
///       .with_writer(writer)
///       .init();
/// 
///    tracing::info!("Hello, world!");
/// }
pub struct Watchman {
    writer: logger::CloudWatchWriter,
}

impl Watchman {
    pub async fn new(sdk_config: SdkConfig, log_group_name: String, log_stream_name: String) -> Self {
        let client = CloudWatchClient::new(sdk_config, log_group_name, log_stream_name);
        match &client.init().await {
            Ok(_) => {},
            Err(_) => {},
        }
        let cloudwatch_client = Arc::new(client);
        let writer = logger::CloudWatchWriter::new(cloudwatch_client.clone());
        Self {
            writer,
        }
    }

    
    pub fn get_writer(&self) -> logger::CloudWatchWriter {
        self.writer.clone()
    }
}


#[cfg(test)]
mod tests {

    use crate::Watchman;

    #[tokio::test]
    async fn tracing_test() {
        let sdk_config = aws_config::load_from_env().await;
        let writer = Watchman::new(sdk_config, "test".to_string(), "new".to_string()).await.get_writer();
        tracing_subscriber::fmt()
            .with_max_level(tracing::Level::INFO)
            .with_target(false)
            .with_writer(writer)
            .init();

        tracing::info!("Hello, world!");
    }
}
