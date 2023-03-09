use alloc::string::String;
use alloc::vec::Vec;
// TODO: Support wasmer when it becomes no_std
use wasmi::*;
use anyhow::{Result, Error};
use hashbrown::HashMap;

pub struct WasmModule {
    module: Module,
    store: Store<()>,
    instance: InstancePre,
}

impl WasmModule {
    /// Runs the module to completion
    /// Yields when calling a function returns a Resumable error
    /// NOTE: Out of fuel trap is not resumable! Only host errors are resumable.
    ///       See: https://github.com/paritytech/wasmi/issues/696
    pub async fn run(mut self) {
        use crate::task_system::task::yield_now;
        let instance = self.instance.ensure_no_start(&mut self.store).expect("Failed to start instance!");
        let entry_point = instance.get_typed_func::<(), ()>(&self.store, "_start").expect("Failed to get `_start` function!");
        let values: [Value; 0] = [];
        let mut call_result = entry_point.call_resumable(&mut self.store, ()).map_err(|e| wasmi::Error::from(e));
        loop {
            if call_result.is_err() {
                error!("WASM trap encountered: {:?}", call_result);
                return;
            } else {
                yield_now().await;
                self.store.add_fuel(1_000_000);
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
    pub(crate) store: Store<()>,

    functions: HashMap<(String, String), Func>,
}

fn yield_now() -> Result<(), wasmi::core::Trap> {
    Err(wasmi::core::Trap::from(super::abi::YieldError))
}

fn add_default_funcs(mut builder: ModuleBuilder) -> ModuleBuilder {
    let func = Func::wrap(&mut builder.store, |caller: Caller<'_, ()>| yield_now());
    let builder = builder.with_func("env", "yield_now", func);
    builder
}

impl ModuleBuilder {
    pub fn from_wasm_bytes(data: &[u8]) -> Result<Self> {
        let mut config = Config::default();
        config.consume_fuel(true);
        let engine = Engine::new(&config);
        let module = Module::new(&engine, data).map_err(Error::msg)?;
        let mut store = Store::new(&engine, ());
        store.add_fuel(10_000_000);

        let obj = Self {
            module,
            store,

            functions: HashMap::new(),
        };
        let obj = add_default_funcs(obj);
        Ok(obj)
    }

    pub fn build(self) -> super::WasmProgram {
        let mut linker: Linker<()> = Linker::new();
        for ((namespace, name), func) in self.functions {
            linker.define(&namespace, &name, func).expect("Failed to define function in wasm linker!");
        }
        let mut store = self.store;
        let module = self.module;
        let instance = linker
            .instantiate(&mut store, &module).expect("Failed to instantiate instance!");

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

    pub fn with_abi(self, abi: &'static impl super::abi::AbiFuncIter) -> Self {
        abi.write_to_builder(self)
    }
}
