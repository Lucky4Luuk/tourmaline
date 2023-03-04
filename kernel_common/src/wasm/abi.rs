use alloc::vec::Vec;
use alloc::boxed::Box;
use alloc::string::String;
use wasmi::core::{HostError, Trap};
use wasmi::{Store, Func, Caller, IntoFunc, AsContextMut};

pub use super::abi_trait::Abi;

#[repr(C)]
#[derive(Debug)]
pub struct Ciov {
    pub ptr: u32,
    pub len: u32,
}

#[derive(Debug)]
pub enum ContextError {
    MemoryNotFound,
    MemoryReadOutOfBounds,
}

pub struct Context<'a> {
    caller: Caller<'a, ()>,
}

impl<'a> Context<'a> {
    pub(crate) fn from_caller(caller: Caller<'a, ()>) -> Self {
        Self {
            caller,
        }
    }

    pub fn read_memory(&mut self, addr: usize, len: usize) -> Result<&[u8], ContextError> {
        let memory = self.caller.get_export("memory").map(|export| export.into_memory()).flatten().ok_or(ContextError::MemoryNotFound)?;
        let bytes: &[u8] = &memory.data_mut(self.caller.as_context_mut()).get(addr .. (addr + len)).ok_or(ContextError::MemoryReadOutOfBounds)?;
        Ok(bytes)
    }

    pub fn read_memory_with_ciovs(&mut self, ciovs: Vec<Ciov>) -> Result<Vec<u8>, ContextError> {
        let mut result = Vec::new();
        for ciov in ciovs {
            let addr = ciov.ptr as usize;
            let len = ciov.len as usize;
            let bytes: &[u8] = self.read_memory(addr, len)?;
            result.extend_from_slice(bytes);
        }
        Ok(result)
    }
}

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
