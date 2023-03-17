pub mod driver;

#[link(wasm_import_module = "sys_abi")]
extern "C" {
    pub fn yield_now();
    pub fn poll_promise(promise_id: i32) -> i32;
}
