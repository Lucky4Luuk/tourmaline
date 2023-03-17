use alloc::vec::Vec;
use alloc::boxed::Box;
use alloc::string::ToString;
use kernel_common::wasm::abi::{
    Context,
    Ciov,
    Abi as AbiTrait
};
use kernel_common::services::{service_manager, ArcMessage};
use kernel_common::driver_common::DriverCommand;
use kernel_common::Promise;

pub struct Abi;

impl Abi {
    const fn new() -> Self {
        Self {}
    }
}

impl AbiTrait for Abi {
    fn driver_write(&self, mut context: Context, name_ptr: i32, name_len: i32, cmd: i32, data_ptr: i32, data_len: i32) -> i32 {
        let name = {
            let name_bytes = context.read_memory(name_ptr as usize, name_len as usize).unwrap();
            core::str::from_utf8(name_bytes).unwrap().to_string()
        };
        let data = {
            let data_bytes = context.read_memory(data_ptr as usize, data_len as usize).unwrap();
            data_bytes.to_vec()
        };

        let promise = Promise::new();
        let promise_id = context.store_promise(promise.clone());

        let message = DriverCommand::func(promise, name, cmd as u8, data);
        let message = ArcMessage::new(Box::new(message));
        service_manager().route_message(message).unwrap();

        promise_id
    }

    fn driver_read(&self, mut context: Context, name_ptr: i32, name_len: i32, cmd: i32, data_ptr: i32, data_len: i32) -> i32 {
        let name = {
            let name_bytes = context.read_memory(name_ptr as usize, name_len as usize).unwrap();
            core::str::from_utf8(name_bytes).unwrap().to_string()
        };
        // let data = {
        //     let data_bytes = context.read_memory(data_ptr as usize, data_len as usize).unwrap();
        //     data_bytes.to_vec()
        // };
        todo!()
    }

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
        kernel_common::services::service_manager().route_message(ArcMessage::new(Box::new(message)));
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
