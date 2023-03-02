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
    fn int3(&self) { trace!("int3!!!"); }

    fn sys_log(&self, data: &[u8]) {
        let msg = core::str::from_utf8(data).unwrap();
        trace!("[WASM] {}", strip_trailing_newline(msg));
    }

    fn stdout(&self) -> Handle {
        2
    }
}

pub static ABI: Abi = Abi::new();
