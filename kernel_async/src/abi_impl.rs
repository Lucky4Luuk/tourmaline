use alloc::vec::Vec;
use kernel_common::wasm::abi::{
    Context,
    Ciov,
    Abi as AbiTrait
};

pub struct Abi;

impl Abi {
    const fn new() -> Self {
        Self {}
    }
}

impl AbiTrait for Abi {
    // Offset0 is where the result will be written.
    // Returns the amount of bytes written.
    fn fd_write(&self, mut context: Context, fd: i32, ciov_buf: i32, ciov_buf_len: i32, offset0: i32) -> i32 {
        let ciov_bytes = context.read_memory(ciov_buf as usize, ciov_buf_len as usize * core::mem::size_of::<Ciov>()).unwrap();
        let ciovs: Vec<Ciov> = ciov_bytes.chunks_exact(8).map(|bytes| {
            let ptr = u32::from_le_bytes([bytes[0], bytes[1], bytes[2], bytes[3]]);
            let len = u32::from_le_bytes([bytes[4], bytes[5], bytes[6], bytes[7]]);
            Ciov { ptr, len }
        }).collect();
        let read_data = context.read_memory_with_ciovs(ciovs).unwrap();
        let read_data = read_data.to_vec();
        let written_bytes = read_data.len() as i32;

        let message = crate::services::FdMessage::fd_write(fd, read_data);
        kernel_common::services::service_manager().route_message(kernel_common::services::ArcMessage::new(alloc::boxed::Box::new(message)));
        context.write_memory(offset0 as usize, &i32::to_le_bytes(written_bytes));
        0 // 0 = Success in ErrNo
    }

    fn environ_sizes_get(&self, mut context: Context, offset0: i32, offset1: i32) -> i32 {
        context.write_memory(offset0 as usize, &[0; 1]);
        context.write_memory(offset1 as usize, &[0; 1]);
        0
    }

    fn environ_get(&self, context: Context, environ: i32, environ_buf: i32) -> i32 {
        unimplemented!("environ_get")
    }
}

pub static ABI: Abi = Abi::new();
