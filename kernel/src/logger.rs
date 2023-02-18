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
        metadata.level() <= Level::Info
    }

    fn log(&self, record: &Record) {
        if self.enabled(record.metadata()) {
            // println!("{} - {}", record.level(), record.args());
            let level = record.level();
            let level_color = match level {
                Level::Error => [255, 48, 64],
                Level::Warn => [222, 144, 27],
                Level::Info => [50, 168, 82],
                _ => [255, 255, 255],
            };
            let spacing = match level {
                Level::Warn => true,
                Level::Info => true,
                _ => false,
            };
            let mut buf = [0u8; 128];
            let mut lock = self.y.lock(); // This lock also synchronizes printing
            let fb = framebuffer::fb_mut();
            if *lock >= fb.height() {
                *lock = 0;
                fb.clear();
            }
            let mut area = framebuffer::Rect::new(0, *lock, fb.width(), fb.height());
            let (width, delta_height) = fb.print(&area, level_color, false, util::show(&mut buf, format_args!("[{}] ", level.as_str())).unwrap_or("FMT FAILED"));
            area.x0 = width;
            area.y0 += delta_height;
            if spacing {
                let (width, delta_height) = fb.print(&area, level_color, false, " ");
                area.x0 = width;
                area.y0 += delta_height;
            }
            let (_, delta_height) = fb.print(&area, [255, 255, 255], false, util::show(&mut buf, format_args!("{}\n", record.args())).unwrap_or("FMT FAILED\n"));
            area.y0 += delta_height;
            *lock = area.y0;
        }
    }

    fn flush(&self) {}
}

static LOGGER: KernelLogger = KernelLogger::new();

pub fn init(level_filter: LevelFilter) -> Result<(), SetLoggerError> {
    log::set_logger(&LOGGER)
        .map(|()| log::set_max_level(level_filter))
}
