use crate::vec::Vec;
use crate::string::String;
// use crate::collections::VecDeque;

static mut HANDLE_STDOUT: Option<Stdout> = None;

pub struct Stdout {
    handle: crate::abi::Handle,
}

impl Stdout {
    pub fn new() -> Self {
        Self {
            handle: crate::abi::abi_stdout(),
        }
    }

    pub fn write(&mut self, msg: String) {
        let bytes: Vec<u8> = msg.bytes().collect();
        crate::abi::abi_handle_write(self.handle, &bytes);
    }
}

pub fn _print(args: crate::fmt::Arguments) {
    if let Some(handle_stdout) = unsafe { HANDLE_STDOUT.as_mut() } {
        let msg = format!("{}", args);
        handle_stdout.write(msg);
    }
}

pub fn init_stdout() {
    unsafe {
        HANDLE_STDOUT = Some(Stdout::new());
    }
}
