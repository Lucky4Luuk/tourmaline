//! A basic framebuffer driver to display things in our operating system.
//! Does not rely on any other part of the ecosystem, and hopefully in
//! the future can be compiled to WASM like all kernels should be.
//! For now, however, the framebuffer driver performance is too important
//! to have it written in WASM, because at the time of writing this, we still
//! rely on `wasmi` for interpreting WASM code, which is not super fast.

#![no_std]

extern crate alloc;

use alloc::string::String;

use kernel_common::Mutex;
use kernel_common::driver_common::*;

mod framebuffer;
use framebuffer::*;
pub use framebuffer::{PixelFormat, FramebufferInfo};

pub struct FramebufferDriver {
    fb: Mutex<Framebuffer>,
}

impl FramebufferDriver {
    pub fn init(fb: &'static mut [u8], info: FramebufferInfo) -> Self {
        Self {
            fb: Mutex::new(Framebuffer::new(fb, info)),
        }
    }

    fn set_pixel(&self, args: &[u8]) -> Result<(), DriverError> {
        let pos_x: u32 = u32::from_le_bytes([
            args.get(0).map(|v| *v).ok_or(DriverError::IncorrectArguments)?,
            args.get(1).map(|v| *v).ok_or(DriverError::IncorrectArguments)?,
            args.get(2).map(|v| *v).ok_or(DriverError::IncorrectArguments)?,
            args.get(3).map(|v| *v).ok_or(DriverError::IncorrectArguments)?,
        ]);

        let pos_y: u32 = u32::from_le_bytes([
            args.get(4).map(|v| *v).ok_or(DriverError::IncorrectArguments)?,
            args.get(5).map(|v| *v).ok_or(DriverError::IncorrectArguments)?,
            args.get(6).map(|v| *v).ok_or(DriverError::IncorrectArguments)?,
            args.get(7).map(|v| *v).ok_or(DriverError::IncorrectArguments)?,
        ]);

        let r: u8 = args.get(8).map(|v| *v).ok_or(DriverError::IncorrectArguments)?;
        let g: u8 = args.get(9).map(|v| *v).ok_or(DriverError::IncorrectArguments)?;
        let b: u8 = args.get(10).map(|v| *v).ok_or(DriverError::IncorrectArguments)?;

        self.fb.lock().set_pixel(pos_x as usize, pos_y as usize, [r,g,b]);

        Ok(())
    }

    fn process_func(&self, func: &Func) -> Result<(), DriverError> {
        match func.func_id() {
            1 => self.set_pixel(func.args()),
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
