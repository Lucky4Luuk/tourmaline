use alloc::string::String;
use kernel_common::services::*;

/// A temporary kernel service for logging stdout to syslogs
pub struct StdoutSyslog;

impl Service for StdoutSyslog {
    fn name(&self) -> String { String::from("stdout_syslog") }

    fn push_message(&self, message: ArcMessage) {
        let text = message.data_as_str().unwrap();
        info!("TEXT!@!! {text}");
    }
}

pub struct StdoutSyslogMessage {
    data: String,
}

impl StdoutSyslogMessage {
    pub fn new(msg: impl Into<String>) -> Self {
        Self {
            data: msg.into(),
        }
    }
}

impl Message for StdoutSyslogMessage {
    fn target(&self) -> &str { "stdout_syslog" }
    fn data(&self) -> &[u8] { self.data.as_bytes() }
    fn data_as_str(&self) -> Option<&str> { Some(&self.data) }
}
