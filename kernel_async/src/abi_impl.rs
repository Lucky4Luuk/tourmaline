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

        let message = crate::services::FdMessage::fd_write(fd, read_data.to_vec());
        kernel_common::services::service_manager().route_message(kernel_common::services::ArcMessage::new(alloc::boxed::Box::new(message)));
        0

        // Special case for stdout
        // if fd == 1 {
        //     let text = core::str::from_utf8(&read_data).unwrap();
        //     let message = crate::services::StdoutSyslogMessage::new(text);
        //     kernel_common::services::service_manager().route_message(kernel_common::services::ArcMessage::new(alloc::boxed::Box::new(message))).unwrap();
        //     0
        // } else {
        //     unimplemented!("fd_write with fd {fd}")
        // }
    }

    fn environ_sizes_get(&self, caller: Context, offset0: i32, offset1: i32) -> i32 {
        0
    }

    fn environ_get(&self, caller: Context, environ: i32, environ_buf: i32) -> i32 {
        0
    }
}

pub static ABI: Abi = Abi::new();
