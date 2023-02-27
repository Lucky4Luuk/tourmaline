extern {
    fn yield_to_kernel();
    fn int3();
    fn kernel_log(data_ptr: *const u8, data_len: u32);
}

pub fn abi_int3() {
    unsafe { int3(); }
}

pub fn abi_yield_to_kernel() {
    unsafe { yield_to_kernel(); }
}

pub fn abi_kernel_log(s: String) {
    let bytes: Vec<u8> = s.bytes().into_iter().collect();
    unsafe { kernel_log(bytes.as_ptr(), bytes.len() as u32); }
}
