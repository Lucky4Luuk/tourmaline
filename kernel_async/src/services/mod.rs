mod file_descriptor_manager;
pub use file_descriptor_manager::*;

use alloc::string::String;
use kernel_common::services::*;

fn strip_trailing_newline(input: &str) -> &str {
    input
        .strip_suffix("\r\n")
        .or(input.strip_suffix("\n"))
        .unwrap_or(input)
}

/// A temporary kernel service for logging stdout to syslogs
pub struct StdoutSyslog;

impl Service for StdoutSyslog {
    fn name(&self) -> String { String::from("stdout_syslog") }

    fn push_message(&self, message: ArcMessage) {
        if let Some(message) = message.as_any().downcast_ref::<StdoutSyslogMessage>() {
            let text = strip_trailing_newline(message.data_as_str().unwrap());
            if message.is_err {
                error!("[STDERR] {text}");
            } else {
                info!("[STDOUT] {text}");
            }
        }
    }
}

pub struct StdoutSyslogMessage {
    data: String,
    is_err: bool,
}

impl StdoutSyslogMessage {
    pub fn new(msg: impl Into<String>) -> ArcMessage {
        Self::new_with_err(msg, false)
    }

    pub fn new_err(msg: impl Into<String>) -> ArcMessage {
        Self::new_with_err(msg, true)
    }

    pub fn new_with_err(msg: impl Into<String>, is_err: bool) -> ArcMessage {
        ArcMessage::new(alloc::boxed::Box::new(Self {
            data: msg.into(),
            is_err,
        }))
    }
}

impl Message for StdoutSyslogMessage {
    fn target(&self) -> &str { "stdout_syslog" }
    fn data_as_str(&self) -> Option<&str> { Some(&self.data) }
}
