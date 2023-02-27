extern {
    fn yield_to_kernel();
    fn int3();
}

fn abi_int3() {
    unsafe { int3(); }
}

fn abi_yield_to_kernel() {
    unsafe { yield_to_kernel(); }
}

#[no_mangle]
pub extern fn start() {
    for i in 0..10 {
        abi_int3();
        // abi_yield_to_kernel();
    }
}
