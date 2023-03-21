use alloc::vec::Vec;
use alloc::string::String;
use wasmi::*;
use anyhow::{Result, Error};
use hashbrown::HashMap;

use crate::Promise;

pub struct ProgStorage {
    promises: Vec<Option<Promise>>,
}

impl ProgStorage {
    pub fn new() -> Self {
        Self {
            promises: Vec::new(),
        }
    }

    pub fn store_promise(&mut self, promise: Promise) -> i32 {
        for (i, maybe_promise) in self.promises.iter_mut().enumerate() {
            if maybe_promise.is_none() {
                *maybe_promise = Some(promise);
                return i as i32;
            }
        }

        // We can only reach this code if no empty slot was found
        // In this case, we just want to grow the buffer
        self.promises.push(Some(promise));
        (self.promises.len() - 1) as i32
    }

    /// Returns:
    /// Result<value, error>
    /// Value maps as following:
    /// =0 = Pending
    /// =1 = Promise does not exist
    /// >1 = Ready(n - 2)
    pub fn poll_promise(&mut self, promise_id: i32) -> i32 {
        if let Some(Some(promise)) = self.promises.get(promise_id as usize) {
            if let Some(value) = promise.poll() {
                if value >= 0 {
                    value + 2
                } else {
                    value
                }
            } else {
                0 // Pending
            }
        } else {
            1 // Promise does not exist
        }
    }
}

pub struct WasmModule {
    module: Module,
    store: Store<ProgStorage>,
    instance: InstancePre,
}

impl WasmModule {
    /// Runs the module to completion
    /// Yields when calling a function returns a Resumable error
    /// NOTE: Out of fuel trap is not resumable! Only host errors are resumable
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
                match self.store.consume_fuel(0) {
                    Ok(remaining_fuel) => {
                        // Make sure we are always above 1 million fuel
                        if remaining_fuel < 10_000_000 {
                            self.store.add_fuel(10_000_000);
                        }
                    },
                    // Only happens when we have no fuel remaining before consuming fuel
                    Err(_) => { self.store.add_fuel(10_000_000); },
                }
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
    pub(crate) store: Store<ProgStorage>,

    functions: HashMap<(String, String), Func>,
}

impl ModuleBuilder {
    pub fn from_wasm_bytes(data: &[u8]) -> Result<Self> {
        let mut config = Config::default();
        config.consume_fuel(true);
        let engine = Engine::new(&config);
        let module = Module::new(&engine, data).map_err(Error::msg)?;
        let mut store = Store::new(&engine, ProgStorage::new());
        let _ = store.add_fuel(100_000_000);

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
