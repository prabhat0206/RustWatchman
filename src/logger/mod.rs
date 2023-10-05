use tracing_subscriber::fmt::writer::MakeWriter;
use std::{io::{self, Write}, sync::Arc};
use crate::cloudwatch::{CloudWatchClient, CloudWatchClientTrait};

#[derive(Clone)]
pub struct CloudWatchWriter {
    cloudwatch_client: Arc<CloudWatchClient>,
}


impl Write for CloudWatchWriter {
    fn write(&mut self, buf: &[u8]) -> io::Result<usize> {
        let log_str = String::from_utf8_lossy(buf);
        let log_str = log_str.trim();

        let _ = match self.cloudwatch_client.put_log_events(vec![log_str.to_string()]) {
            Ok(_) => {},
            Err(_) => {},
        };
        Ok(buf.len())
    } 

    fn flush(&mut self) -> io::Result<()> {
        Ok(())
    }
}

impl CloudWatchWriter {
    pub fn new(cloudwatch_client: Arc<CloudWatchClient>) -> Self {
        Self { cloudwatch_client }
    }
}

impl MakeWriter<'_> for CloudWatchWriter {
    type Writer = Self;

    fn make_writer(&self) -> Self::Writer {
        Self::new(self.cloudwatch_client.clone())
    }
}
