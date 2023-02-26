use alloc::string::{String, ToString};
use log::{Record, Level, Metadata, SetLoggerError, LevelFilter};
use crate::{framebuffer, util};

struct KernelLogger {
    y: spin::Mutex<usize>,
}

impl KernelLogger {
    pub const fn new() -> Self {
        Self {
            y: spin::Mutex::new(0),
        }
    }
}

impl log::Log for KernelLogger {
    fn enabled(&self, metadata: &Metadata) -> bool {
        // metadata.level() <= Level::Info
        true
    }

    fn log(&self, record: &Record) {
        if self.enabled(record.metadata()) {
            let level = record.level();
            let level_color = match level {
                Level::Error => [255, 48, 64],
                Level::Warn => [222, 144, 27],
                Level::Info => [50, 168, 82],
                Level::Debug => [50, 168, 157],
                _ => [255, 255, 255],
            };
            let spacing = match level {
                Level::Warn => true,
                Level::Info => true,
                _ => false,
            };
            // let size = framebuffer::TextSize::Small;
            let size = framebuffer::TextSize::Normal;
            let mut lock = self.y.lock(); // This lock also synchronizes printing
            let fb = framebuffer::fb_mut();
            if *lock >= fb.height() {
                *lock = 0;
                fb.clear();
            }
            let mut area = framebuffer::Rect::new(0, *lock, fb.width(), fb.height());
            let (width, delta_height) = fb.print(&area, level_color, size, format(format_args!("[{}] ", level.as_str())).as_ref());
            area.x0 = width;
            area.y0 += delta_height;
            if spacing {
                let (width, delta_height) = fb.print(&area, level_color, size, " ");
                area.x0 = width;
                area.y0 += delta_height;
            }
            let (_, delta_height) = fb.print(&area, [255, 255, 255], size, format(format_args!("{}\n", record.args())).as_ref());
            area.y0 += delta_height;
            *lock = area.y0;
        }
    }

    fn flush(&self) {}
}

enum Formatted<'a> {
    String(alloc::string::String),
    Lazy(([u8; 2048], core::fmt::Arguments<'a>))
}

impl<'a> Formatted<'a> {
    fn as_ref(&mut self) -> &str {
        match self {
            Self::String(value) => &*value,
            Self::Lazy((buf, args)) => {
                util::show(buf, format_args!("{}", args)).expect("Failed to format string!")
            },
        }
    }
}

fn format(args: core::fmt::Arguments) -> Formatted {
    use alloc::format;
    if crate::heap::is_initialized() {
        Formatted::String(format!("{}", args))
    } else {
        Formatted::Lazy(([0u8; 2048], args))
    }
}

static LOGGER: KernelLogger = KernelLogger::new();

pub fn init(level_filter: LevelFilter) -> Result<(), SetLoggerError> {
    log::set_logger(&LOGGER)
        .map(|()| log::set_max_level(level_filter))
}
