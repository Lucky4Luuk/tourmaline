use alloc::vec::Vec;
use alloc::string::String;
use wasmi::core::HostError;
use wasmi::{Store, Func, Caller, IntoFunc, AsContextMut, Memory};

use super::backend::ProgStorage;
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
    caller: Caller<'a, ProgStorage>,
}

impl<'a> core::ops::Deref for Context<'a> {
    type Target = ProgStorage;
    fn deref(&self) -> &Self::Target {
        self.caller.data()
    }
}

impl<'a> core::ops::DerefMut for Context<'a> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        self.caller.data_mut()
    }
}

impl<'a> Context<'a> {
    pub(crate) fn from_caller(caller: Caller<'a, ProgStorage>) -> Self {
        Self {
            caller,
        }
    }

    fn memory(&self) -> Result<Memory, ContextError> {
        self.caller.get_export("memory").map(|export| export.into_memory()).flatten().ok_or(ContextError::MemoryNotFound)
    }

    pub fn write_memory(&mut self, addr: usize, data: &[u8]) -> Result<(), ContextError> {
        let memory = self.memory()?;
        let bytes = memory.data_mut(self.caller.as_context_mut()).get_mut(addr .. (addr + data.len())).ok_or(ContextError::MemoryReadOutOfBounds)?;
        bytes.copy_from_slice(data);
        Ok(())
    }

    pub fn read_memory(&mut self, addr: usize, len: usize) -> Result<&[u8], ContextError> {
        let memory = self.memory()?;
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
    env: String,
    name: String,
    func: Func,
}

impl AbiFunc {
    pub fn wrap<Params, Results>(env: impl Into<String>, name: impl Into<String>, store: &mut Store<ProgStorage>, f: impl IntoFunc<ProgStorage, Params, Results>) -> Self {
        Self {
            env: env.into(),
            name: name.into(),
            func: Func::wrap::<ProgStorage, Params, Results>(store, f),
        }
    }
}

pub trait AbiFuncIter: Abi {
    fn functions(&'static self, store: &mut Store<ProgStorage>) -> Vec<AbiFunc> {
        include!("code_gen.rs")
    }

    fn write_to_builder(&'static self, mut builder: super::backend::ModuleBuilder) -> super::backend::ModuleBuilder {
        for func in self.functions(&mut builder.store) {
            builder = builder.with_func(func.env, func.name, func.func);
        }
        builder
    }
}

impl<T: Abi> AbiFuncIter for T {}

pub(crate) fn yield_now() -> Result<(), wasmi::core::Trap> {
    Err(wasmi::core::Trap::from(YieldError))
}

#[derive(Debug, Copy, Clone)]
pub struct YieldError;

impl core::fmt::Display for YieldError {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        write!(f, "YieldError")
    }
}

impl HostError for YieldError {}
