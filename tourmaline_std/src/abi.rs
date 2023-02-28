use crate::string::String;
use crate::vec::Vec;

pub type Handle = u32;

extern {
    fn yield_to_kernel();
    fn int3();

    fn sys_log(data_ptr: *const u8, data_len: u32);

    fn stdout() -> Handle;

    fn handle_close(handle: Handle);
    fn handle_write(handle: Handle, data: *const u8, data_len: u32);
    fn handle_read(handle: Handle, data: *mut u8, data_len: u32);
}

pub fn abi_int3() {
    unsafe { int3(); }
}

pub fn abi_yield_to_kernel() {
    unsafe { yield_to_kernel(); }
}

pub fn abi_sys_log(s: String) {
    let bytes: Vec<u8> = s.bytes().into_iter().collect();
    unsafe { sys_log(bytes.as_ptr(), bytes.len() as u32); }
}

pub fn abi_stdout() -> Handle {
    unsafe { stdout() }
}

pub fn abi_handle_write(handle: Handle, data: &[u8]) {
    unsafe {
        handle_write(handle, data.as_ptr(), data.len() as u32);
    }
}
