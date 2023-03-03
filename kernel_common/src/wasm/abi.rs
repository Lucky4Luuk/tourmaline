use alloc::vec::Vec;
use alloc::boxed::Box;
use alloc::string::String;
use wasmi::core::{HostError, Trap};
use wasmi::{Store, Func, Caller, IntoFunc};

pub use super::abi_trait::Abi;

use super::async_bridge::AbiAsyncBridge;

pub type Handle = u32;

pub struct AbiFunc {
    name: String,
    func: Func,
}

impl AbiFunc {
    pub fn wrap<Params, Results>(name: impl Into<String>, store: &mut Store<()>, f: impl IntoFunc<(), Params, Results>) -> Self {
        Self {
            name: name.into(),
            func: Func::wrap::<(), Params, Results>(store, f),
        }
    }
}

pub trait AbiFuncIter: Abi {
    fn functions(&'static self, store: &mut Store<()>) -> Vec<AbiFunc> {
        include!("code_gen.rs")
    }

    fn write_to_builder(&'static self, mut builder: super::backend::ModuleBuilder) -> super::backend::ModuleBuilder {
        for func in self.functions(&mut builder.store) {
            builder = builder.with_func("wasi_snapshot_preview1", func.name, func.func);
        }
        builder
    }
}

impl<T: Abi> AbiFuncIter for T {}

#[derive(Debug, Copy, Clone)]
pub struct YieldError;

impl core::fmt::Display for YieldError {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        write!(f, "YieldError")
    }
}

impl HostError for YieldError {}
