pub mod promise;
pub mod abi;

use promise::Promise;
use abi::driver::safe_driver_write;

pub fn set_pixel(x: u32, y: u32, r: u8, g: u8, b: u8) -> Promise {
    let name = String::from("FRAMEBUFFER_DRIVER");
    let cmd = 1; // set pixel
    let mut data = Vec::new();
    data.extend_from_slice(&u32::to_le_bytes(x));
    data.extend_from_slice(&u32::to_le_bytes(y));
    data.push(r);
    data.push(g);
    data.push(b);
    safe_driver_write(name, cmd, data)
}
