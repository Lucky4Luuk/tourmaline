use kernel_common::wasm::abi::Abi as AbiTrait;
use kernel_common::wasm::abi::Handle;

fn strip_trailing_newline(input: &str) -> &str {
    input
        .strip_suffix("\r\n")
        .or(input.strip_suffix("\n"))
        .unwrap_or(input)
}

pub enum HandleType {
    Null,
    Stdout,
}

pub struct AbiHandleMap {
    next_handle_id: u32,
    handle_map: hashbrown::HashMap<Handle, HandleType>,
}

impl AbiHandleMap {
    pub fn new() -> Self {
        Self {
            next_handle_id: 1,
            handle_map: hashbrown::HashMap::new(),
        }
    }

    pub fn stdout(&mut self) -> Handle {
        let handle = self.next_handle_id;
        self.next_handle_id.checked_add(1).expect("Ran out of handle ids!");
        self.handle_map.insert(handle, HandleType::Stdout);
        handle
    }
}

pub struct Abi;

impl Abi {
    const fn new() -> Self {
        Self {}
    }
}

impl AbiTrait for Abi {
    fn int3(&self) { trace!("int3!!!"); }

    fn sys_log(&self, data: &[u8]) {
        let msg = core::str::from_utf8(data).unwrap();
        trace!("[WASM] {}", strip_trailing_newline(msg));
    }

    fn stdout(&self) -> Handle {
        2
    }
}

pub static ABI: Abi = Abi::new();
