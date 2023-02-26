//! A logger implementation that supports multiple backends.
//! Fair warning: This implementation has not been tested, and I'm not sure
//! if it performs well at all.

use log::{Record, Metadata, LevelFilter};
use conquer_once::spin::Once;

struct KernelLogger {
    internal: spin::Mutex<Option<&'static dyn log::Log>>,
}

impl KernelLogger {
    pub const fn new() -> Self {
        Self {
            internal: spin::Mutex::new(None),
        }
    }

    pub fn set_logger(&self, logger: &'static dyn log::Log) {
        let mut lock = self.internal.lock();
        *lock = Some(logger);
    }
}

impl log::Log for KernelLogger {
    fn enabled(&self, _metadata: &Metadata) -> bool {
        true
    }

    fn log(&self, record: &Record) {
        let lock = self.internal.lock();
        lock.as_ref().expect("No logger initialized!").log(record);
    }

    fn flush(&self) {}
}

static LOGGER: KernelLogger = KernelLogger::new();
static LOGGER_INIT: Once = Once::uninit();

pub fn init(level_filter: LevelFilter, logger: &'static dyn log::Log) {
    LOGGER_INIT.init_once(|| {
        log::set_logger(&LOGGER)
            .map(|()| log::set_max_level(level_filter))
            .expect("Failed to set up log!");
    });
    LOGGER.set_logger(logger);
}
