use alloc::string::String;
use alloc::vec::Vec;
// TODO: Support wasmer when it becomes no_std
use wasmi::*;
use anyhow::{Result, Error};
use hashbrown::HashMap;

pub struct WasmModule {
    module: Module,
    store: Store<()>,
    instance: Instance,
}

impl WasmModule {
    /// Runs the module to completion
    /// Yields when calling a function returns a Trap::OutOfFuel error
    pub async fn run(&mut self) {
        use crate::task_system::task::yield_now;

        let entry_point = self.instance.get_typed_func::<(), ()>(&self.store, "start").expect("Failed to get `main` function!");

        let values: Vec<Value> = Vec::new();
        let mut call_result = entry_point.call_resumable(&mut self.store, ()).map_err(|e| wasmi::Error::from(e));
        loop {
            if call_result.is_err() {
                panic!("WASM trap encountered: {:?}", call_result);
            } else {
                yield_now().await;
                if let TypedResumableCall::Resumable(call) = call_result.unwrap() {
                    call_result = call.resume(&mut self.store, &values[..]);
                } else {
                    return;
                }
            }
        }
    }
}

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

    pub fn build(self) -> super::WasmProgram {
        let mut linker: Linker<()> = Linker::new();
        for ((namespace, name), func) in self.functions {
            linker.define(&namespace, &name, func).expect("Failed to define function in wasm linker!");
        }
        let mut store = self.store;
        let module = self.module;
        let instance = linker
            .instantiate(&mut store, &module).expect("Failed to instantiate instance!")
            .start(&mut store).expect("Failed to start instance!");

        let wasm_module = WasmModule {
            module,
            store,
            instance,
        };

        super::WasmProgram::from_module(wasm_module)
    }

    pub fn with_func(mut self, namespace: impl Into<String>, name: impl Into<String>, func: Func) -> Self {
        self.functions.insert((namespace.into(), name.into()), func);
        self
    }

    pub fn with_abi(mut self, abi: &'static impl super::abi::Abi) -> Self {
        let func = Func::wrap(&mut self.store, |_caller: Caller<'_, ()>| abi.yield_to_kernel());
        self = self.with_func("env", "yield_to_kernel", func);
        let func = Func::wrap(&mut self.store, |_caller: Caller<'_, ()>| abi.int3());
        self = self.with_func("env", "int3", func);
        let func = Func::wrap(&mut self.store, |mut caller: Caller<'_, ()>, data_ptr: i32, data_len: u32| {
            let memory = caller.get_export("memory").map(|export| export.into_memory()).flatten().unwrap();
            let bytes: &[u8] = &memory.data_mut(&mut caller)[data_ptr as usize .. (data_ptr as usize + data_len as usize)];
            abi.log(bytes)
        });
        self = self.with_func("env", "kernel_log", func);
        self
    }
}
