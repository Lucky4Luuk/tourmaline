//! A basic framebuffer driver to display things in our operating system.
//! Does not rely on any other part of the ecosystem, and hopefully in
//! the future can be compiled to WASM like all kernels should be.
//! For now, however, the framebuffer driver performance is too important
//! to have it written in WASM, because at the time of writing this, we still
//! rely on `wasmi` for interpreting WASM code, which is not super fast.

#![no_std]

extern crate alloc;

use alloc::string::String;

use kernel_common::driver_common::*;

mod framebuffer;
use framebuffer::*;
pub use framebuffer::{PixelFormat, FramebufferInfo};

pub struct FramebufferDriver {
    fb: Framebuffer,
}

impl FramebufferDriver {
    pub fn init(fb: &'static mut [u8], info: FramebufferInfo) -> Self {
        Self {
            fb: Framebuffer::new(fb, info),
        }
    }

    fn process_func(&self, func: &Func) -> Result<(), DriverError> {
        match func.func_id() {
            _ => Err(DriverError::UnsupportedFunc)
        }
    }
}

impl Driver for FramebufferDriver {
    fn name(&self) -> String { String::from("FRAMEBUFFER_DRIVER") }

    fn process_command(&self, cmd: &DriverCommand) {
        let result = match &cmd.inner {
            Cmd::Func(func) => self.process_func(func),
            _ => Err(DriverError::UnsupportedCommand),
        };
        let v = match result {
            Ok(()) => 1,
            Err(e) => (e as u8 as i32 * -1) - 1,
        };
        cmd.promise.complete(v);
    }
}
