//! A logger implementation for the synchronous part of the kernel.
//! This simply logs messages to the framebuffer.

use log::{Record, Level, Metadata};
use crate::{framebuffer, util};

pub struct FramebufferLogger {
    y: spin::Mutex<usize>,
}

impl FramebufferLogger {
    pub const fn new() -> Self {
        Self {
            y: spin::Mutex::new(0),
        }
    }
}

impl log::Log for FramebufferLogger {
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

            let mut lock = self.y.lock();
            let mut fb = framebuffer::fb_mut();

            // Prescroll based on newlines and window width
            let char_width = size.char_width();
            let newline_count = format(format_args!("{}\n", record.args())).count_newlines((fb.width() / char_width) + 1) + 1;
            let expected_height = size.as_u8() as usize * newline_count;
            if *lock > expected_height && *lock + expected_height >= fb.height() {
                *lock -= expected_height;
                fb.scroll_up(expected_height);
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

    fn count_char(&mut self, c: char) -> usize {
        self.as_ref().chars().filter(|s| *s == c).count()
    }

    /// fb_width is in characters
    fn count_newlines(&mut self, fb_width: usize) -> usize {
        let mut count = 0;
        for line in self.as_ref().lines() {
            let split_count = line.chars().count() / fb_width;
            count += 1 + split_count;
        }
        count
    }
}

fn format(args: core::fmt::Arguments) -> Formatted {
    if crate::heap::is_initialized() {
        Formatted::String(format!("{}", args))
    } else {
        Formatted::Lazy(([0u8; 2048], args))
    }
}

pub static LOGGER: FramebufferLogger = FramebufferLogger::new();
