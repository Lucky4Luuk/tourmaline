//! Common items shared by both the sync and async kernel stages.

#![no_std]

#[macro_use] extern crate alloc;
#[macro_use] extern crate log;

use core::marker::Sized;

pub mod logger;
pub mod task_system;
pub mod wasm;

pub trait StaticRef: Sized + 'static {
    const CONST: &'static Self;
    fn static_ref() -> &'static Self {
        Self::CONST
    }
}
