use kernel_common::wasm::abi::Abi as AbiTrait;
use kernel_common::wasm::abi::Handle;

fn strip_trailing_newline(input: &str) -> &str {
    input
        .strip_suffix("\r\n")
        .or(input.strip_suffix("\n"))
        .unwrap_or(input)
}

pub struct Abi;

impl Abi {
    const fn new() -> Self {
        Self {}
    }
}

impl AbiTrait for Abi {
    fn fd_write(&self, fd: i32, ciov_buf: i32, ciov_buf_len: i32, offset0: i32) -> i32 {
        panic!("fd_write");
    }
}

pub static ABI: Abi = Abi::new();
