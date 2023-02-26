//* Logger that logs to a buffer, instead of the framebuffer directly

use alloc::string::{String, ToString};
use alloc::collections::vec_deque::VecDeque;
use log::{Record, Level, Metadata, SetLoggerError, LevelFilter};

pub struct Message {
    level: Level,
    content: String,
}

impl Message {
    fn new(record: &Record) -> Self {
        Self {
            level: record.level(),
            content: format!("{}", record.args()),
        }
    }
}

pub struct BufferLogger {
    messages: spin::Mutex<VecDeque<Message>>,
    max_messages: usize,
}

impl BufferLogger {
    pub const fn new() -> Self {
        Self {
            messages: spin::Mutex::new(VecDeque::new()),
            max_messages: 128,
        }
    }
}

impl log::Log for BufferLogger {
    fn enabled(&self, _metadata: &Metadata) -> bool {
        true
    }

    fn log(&self, record: &Record) {
        if self.enabled(record.metadata()) {
            let mut lock = self.messages.lock();
            lock.push_back(Message::new(record));
        }
    }

    fn flush(&self) {}
}

pub static LOGGER: BufferLogger = BufferLogger::new();
