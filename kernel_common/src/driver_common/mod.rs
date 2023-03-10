//! Contains helper utilities for implementing drivers.

use alloc::string::String;

use crate::services::{Service, ArcMessage};
use crate::wasm::WasmProgram;
use crate::Promise;

mod command;
pub use command::*;

#[derive(Debug)]
#[repr(u8)]
pub enum DriverError {
    UnsupportedCommand = 0,
    UnsupportedFunc,
}

pub trait Driver: Send {
    fn name(&self) -> String;

    /// Should preferably spawn an async task, and complete the promise
    /// stored in the command
    fn process_command(&self, cmd: &DriverCommand);
}

impl<T: Driver + Sync> Service for T {
    fn name(&self) -> String { self.name() }
    fn push_message(&self, message: ArcMessage) {
        if let Some(cmd) = message.as_any().downcast_ref::<DriverCommand>() {
            self.process_command(cmd);
        } else {
            panic!("Message arriving at service is not of type `DriverCommand`!");
        }
    }
}

pub struct WasmDriver {
    name: String,
    prog: WasmProgram,
}

impl Driver for WasmDriver {
    fn name(&self) -> String { self.name.clone() }

    fn process_command(&self, cmd: &DriverCommand) {
        unimplemented!();
    }
}
