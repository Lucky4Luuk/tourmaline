#[macro_use] extern crate tourmaline_std;

use tourmaline_std::{*, abi::*};

#[no_mangle]
pub extern fn start() {
    io::init_stdout();
    for i in 0..2 {
        // abi_int3();
        // println!("number: {}", i);
        kernel_log("logg!!!");
        // TODO: This should not have to be inserted by hand
        //       The wasm backend in the kernel should insert these automatically
        //       or call this based on gas-fees or whatever works!
        // TODO: Or even better, do not require this to be inserted at all! Host function calls
        //       are very expensive (at least in wasmi) so this would hamper performance even more.
        abi_yield_to_kernel();
    }
}
