pub type Handle = u32;

extern {
    fn yield_to_kernel();
    fn int3();

    fn sys_log(data_ptr: *const u8, data_len: u32);

    fn stdout() -> Handle;
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
