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
pub mod rtc;

pub use spin::Mutex;
pub use spin::MutexGuard;

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
