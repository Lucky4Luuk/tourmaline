use alloc::vec::Vec;
use alloc::string::String;

use crate::services::{ArcMessage, Message};
use crate::Promise;

#[derive(Clone)]
pub struct DriverCommand {
    pub target: String,   // Relates directly to `[Driver::name]`
    pub inner: Cmd,
    pub promise: Promise,
}

impl DriverCommand {
    pub fn func(promise: Promise, target: impl Into<String>, func_id: u8, args: Vec<u8>) -> Self {
        Self {
            target: target.into(),
            inner: Cmd::Func(Func::new(func_id, args)),
            promise,
        }
    }
}

impl Message for DriverCommand {
    fn target(&self) -> &str { &self.target }

    fn on_response(&self, message: ArcMessage) {

    }
}

#[derive(Clone)]
pub enum Cmd {
    Func(Func),
}

/// A function call stored as an array of bytes.
/// The function id is used by the driver to figure
/// out what arguments to expect. It is up to the
/// driver to deal with missing arguments or too many arguments.
#[derive(Clone)]
pub struct Func {
    func_id: u8,
    args: Vec<u8>,
}

impl Func {
    pub fn new(func_id: u8, args: Vec<u8>) -> Self {
        Self {
            func_id,
            args
        }
    }

    pub fn func_id(&self) -> u8 {
        self.func_id
    }

    pub fn as_byte_slice(&self) -> &[u8] {
        &self.args
    }
}
