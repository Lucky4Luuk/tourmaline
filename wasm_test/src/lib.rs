use tourmaline_std::{*, abi::*};

#[no_mangle]
pub extern fn start() {
    for i in 0..10 {
        // abi_int3();
        kernel_log("test");
        // TODO: This should not have to be inserted by hand
        //       The wasm backend in the kernel should insert these automatically
        //       or call this based on gas-fees or whatever works!
        abi_yield_to_kernel();
    }
}
