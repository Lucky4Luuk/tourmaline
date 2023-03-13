use std::task::Poll;

extern "C" {
    fn yield_now();
    fn poll_promise(promise_id: i32) -> i32;
}

#[link(wasm_import_module = "driver_abi")]
extern "C" {
    fn call_driver(name_ptr: *const u8, name_len: i32, cmd: i32, data_ptr: *const u8, data_len: i32) -> i32;
}

#[derive(Debug)]
pub enum PromiseError {
    PromiseNotFound,
}

pub struct Promise {
    id: i32,
}

impl Promise {
    pub fn new(id: i32) -> Self {
        Self {
            id,
        }
    }

    pub fn poll(&self) -> Result<Poll<i32>, PromiseError> {
        let v = unsafe { poll_promise(self.id) };
        if v == 0 { return Ok(Poll::Pending); }
        if v == 1 { return Err(PromiseError::PromiseNotFound); }
        if v < 0 { return Ok(Poll::Ready(v)); }
        Ok(Poll::Ready(v - 2))
    }
}

fn safe_call_driver(driver_name: String, cmd: i32, data: Vec<u8>) -> Promise {
    let name_bytes: Vec<u8> = driver_name.bytes().into_iter().collect();
    let promise_id = unsafe { call_driver(name_bytes.as_ptr(), name_bytes.len() as i32, cmd, data.as_ptr(), data.len() as i32) };
    Promise::new(promise_id)
}

fn wait_for(promise: Promise) -> i32 {
    loop {
        match promise.poll() {
            Ok(Poll::Pending) => unsafe { yield_now() },
            Ok(Poll::Ready(value)) => return value,
            Err(e) => panic!("Promise returned an error! {:?}", e),
        }
    }
}

fn set_pixel(x: u32, y: u32, r: u8, g: u8, b: u8) -> Promise {
    let name = String::from("FRAMEBUFFER_DRIVER");
    let cmd = 1; // set pixel
    let mut data = Vec::new();
    data.extend_from_slice(&u32::to_le_bytes(x));
    data.extend_from_slice(&u32::to_le_bytes(y));
    data.push(r);
    data.push(g);
    data.push(b);
    safe_call_driver(name, cmd, data)
}

fn main() {
    println!("Hello, world!");

    for y in 0..255 {
        for x in 0..255 {
            unsafe { yield_now(); }
            let promise = set_pixel(x,y, 255,0,0);
            unsafe { yield_now(); }
            let _ = wait_for(promise);
        }
    }
}
