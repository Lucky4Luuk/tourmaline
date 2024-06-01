use tourmaline_std::{
    promise::wait_for,
    abi::yield_now,
    set_pixel,
};

fn main() {
    println!("Hello, world!");
    println!("I'm running in user space!!!");

    // for y in 0..255 {
    //     for x in 0..255 {
    //         unsafe { yield_now(); }
    //         let promise = set_pixel(x,y, 0, y as u8, x as u8);
    //         unsafe { yield_now(); }
    //         let _ = wait_for(promise);
    //     }
    // }
}
