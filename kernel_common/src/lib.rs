//! Common items shared by both the sync and async kernel stages.

#![no_std]

#[macro_use] extern crate alloc;
#[macro_use] extern crate log;
#[macro_use] extern crate async_trait;

use core::sync::atomic::{AtomicBool, AtomicI32, Ordering};
use alloc::sync::Arc;

pub mod requests;
pub mod logger;
pub mod task_system;
pub mod wasm;
pub mod services;
pub mod driver_common;

/// A promise for a value. Alternative to Rust futures to be used across WASM.
/// Because it cannot be generic over a return value, a promise can only
/// return basic information (in this case, an i32).
#[derive(Clone)]
pub struct Promise {
    signal: Arc<AtomicBool>,
    value: Arc<AtomicI32>,
}

impl Promise {
    pub fn new() -> Self {
        Self {
            signal: Arc::new(AtomicBool::new(false)),
            value: Arc::new(AtomicI32::new(0)),
        }
    }

    pub fn complete(&self, value: i32) {
        self.value.store(value, Ordering::Release);
        self.signal.store(true, Ordering::Relaxed);
    }

    pub fn poll(&self) -> Option<i32> {
        if self.signal.load(Ordering::Relaxed) == true {
            let value = self.value.load(Ordering::Acquire);
            Some(value)
        } else {
            None
        }
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
