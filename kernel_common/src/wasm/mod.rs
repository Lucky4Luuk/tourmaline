mod backend;
mod abi_trait;
pub mod abi;

use backend::{WasmModule, ModuleBuilder};

pub struct WasmProgram {
    module: WasmModule,
}

impl WasmProgram {
    pub fn new(data: &[u8], abi: &'static impl abi::AbiFuncIter) -> Self {
        ModuleBuilder::from_wasm_bytes(data).expect("Failed to create module builder!")
            .with_abi(abi)
            .build()
    }

    pub(crate) fn from_module(module: WasmModule) -> Self {
        Self {
            module,
        }
    }

    pub async fn run(mut self) {
        self.module.run().await;
    }
}
