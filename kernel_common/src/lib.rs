//! Common items shared by both the sync and async kernel stages.

#![no_std]

#[macro_use] extern crate alloc;
#[macro_use] extern crate log;
#[macro_use] extern crate async_trait;

use core::marker::Sized;

pub mod logger;
pub mod task_system;
pub mod wasm;
pub mod framebuffer;
pub mod services;

pub trait StaticRef: Sized + 'static {
    const CONST: &'static Self;
    fn static_ref() -> &'static Self {
        Self::CONST
    }
}

/// Very simple sleep function based on the RTC.
/// It does not read anything more accurate than seconds, so you can only sleep for
/// whole seconds.
/// This function can also break when it runs right over the month/year/century barrier.
/// It does not take into account the current month, year or century.
pub fn rtc_sleep(seconds: u32) {
    use cmos_rtc::{ReadRTC, Time};
    fn time_to_sec(time: Time) -> u32 {
        time.second as u32 + (time.minute as u32) * 60 + (time.hour as u32) * 60 * 60 + (time.day as u32) * 60 * 60 * 24
    }
    let mut cmos = ReadRTC::new(0x00, 0x00);
    let start = cmos.read();
    let start_sec = time_to_sec(start);
    'wait: loop {
        let now = cmos.read();
        if time_to_sec(now) > start_sec + seconds {
            break 'wait;
        }
    }
}
