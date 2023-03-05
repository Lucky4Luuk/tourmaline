use alloc::vec::Vec;
use kernel_common::wasm::abi::{
    Context,
    Ciov,
    Abi as AbiTrait
};

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
    // Offset0 is where the result must be written.
    // Returns the amount of bytes written.
    fn fd_write(&self, mut context: Context, fd: i32, ciov_buf: i32, ciov_buf_len: i32, offset0: i32) -> i32 {
        let ciov_bytes = context.read_memory(ciov_buf as usize, ciov_buf_len as usize * core::mem::size_of::<Ciov>()).unwrap();
        let ciovs: Vec<Ciov> = ciov_bytes.chunks_exact(8).map(|bytes| {
            let ptr = u32::from_le_bytes([bytes[0], bytes[1], bytes[2], bytes[3]]);
            let len = u32::from_le_bytes([bytes[4], bytes[5], bytes[6], bytes[7]]);
            Ciov { ptr, len }
        }).collect();
        let read_data = context.read_memory_with_ciovs(ciovs).unwrap();
        // let text = core::str::from_utf8(&read_data).unwrap();
        // panic!("text: {}", text);
        unimplemented!();
    }
}

pub static ABI: Abi = Abi::new();
