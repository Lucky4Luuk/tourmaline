use crate::promise::Promise;

#[link(wasm_import_module = "driver_abi")]
extern "C" {
    pub fn driver_write(name_ptr: *const u8, name_len: i32, cmd: i32, data_ptr: *const u8, data_len: i32) -> i32;
    pub fn driver_read(name_ptr: *const u8, name_len: i32, cmd: i32, data_ptr: *const u8, data_len: i32) -> i32;
}

pub fn safe_driver_write(driver_name: String, cmd: i32, data: Vec<u8>) -> Promise {
    let name_bytes: Vec<u8> = driver_name.bytes().into_iter().collect();
    let promise_id = unsafe { driver_write(name_bytes.as_ptr(), name_bytes.len() as i32, cmd, data.as_ptr(), data.len() as i32) };
    Promise::new(promise_id)
}
