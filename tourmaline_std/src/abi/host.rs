#[link(wasm_import_module = "host_abi")]
extern "C" {
    pub fn host_memset(addr: i32, data_ptr: i32, data_len: i32) -> i32;
    pub fn host_memread(read_addr: i32, buf_ptr: i32, buf_len: i32) -> i32;
}

pub fn safe_host_memset(addr: i32, data: &[u8]) -> i32 {
    unsafe {
        host_memset(addr, data.as_ptr() as i32, data.len() as i32)
    }
}

pub fn safe_host_memread(read_addr: i32, buf: &mut [u8]) -> i32 {
    unsafe {
        host_memread(read_addr, buf.as_ptr() as i32, buf.len() as i32)
    }
}
