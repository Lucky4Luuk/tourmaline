use alloc::string::String;
// TODO: Support wasmer when it becomes no_std
use wasmi::*;
use anyhow::{Result, Error};
use hashbrown::HashMap;

pub struct ModuleBuilder {
    module: Module,
    store: Store<()>,

    functions: HashMap<(String, String), Func>,
}

impl ModuleBuilder {
    pub fn from_wasm_bytes(data: &[u8]) -> Result<Self> {
        let engine = Engine::default();
        let module = Module::new(&engine, data).map_err(Error::msg)?;
        let store = Store::new(&engine, ());
        Ok(Self {
            module,
            store,

            functions: HashMap::new(),
        })
    }

    pub fn with_func(mut self, namespace: impl Into<String>, name: impl Into<String>, func: Func) -> Self {
        self.functions.insert((namespace.into(), name.into()), func);
        self
    }

    pub fn with_abi(mut self, abi: &'static impl super::abi::Abi) -> Self {
        let func = Func::wrap(&mut self.store, |caller: Caller<'_, ()>| abi.int3());
        self = self.with_func("abi", "int3", func);
        self
    }
}
