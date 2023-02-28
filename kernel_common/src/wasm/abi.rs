use alloc::vec::Vec;
use alloc::string::String;
use wasmi::core::{HostError, Trap};
use wasmi::{Store, Func, Caller, IntoFunc};

pub type Handle = u32;

pub trait Abi: Send + Sync {
    fn yield_to_kernel(&self) -> Result<(), Trap> { Err(YieldError.into()) }
    fn int3(&self);

    fn sys_log(&self, data: &[u8]);

    // Opening certain handles
    fn stdout(&self) -> Handle { todo!("stdout"); }

    // Handle related functions
    fn handle_close(&self, _handle: Handle) { todo!("handle_close"); }
    fn handle_write(&self, _handle: Handle, _data: &[u8]) { todo!("handle_write"); }
}

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
        vec![
            AbiFunc::wrap("yield_to_kernel", store, |_caller: Caller<'_, ()>| self.yield_to_kernel()),
            AbiFunc::wrap("int3", store, |_caller: Caller<'_, ()>| self.int3()),

            AbiFunc::wrap("sys_log", store, |mut caller: Caller<'_, ()>, data_ptr: i32, data_len: u32| {
                let memory = caller.get_export("memory").map(|export| export.into_memory()).flatten().unwrap();
                let bytes: &[u8] = &memory.data_mut(&mut caller)[data_ptr as usize .. (data_ptr as usize + data_len as usize)];
                self.sys_log(bytes);
            }),

            AbiFunc::wrap("stdout", store, |_caller: Caller<'_, ()>| self.stdout()),

            AbiFunc::wrap("handle_close", store, |_caller: Caller<'_, ()>, handle: Handle| self.handle_close(handle)),
            AbiFunc::wrap("handle_write", store, |mut caller: Caller<'_, ()>, handle: Handle, data_ptr: i32, data_len: u32| {
                let memory = caller.get_export("memory").map(|export| export.into_memory()).flatten().unwrap();
                let bytes: &[u8] = &memory.data_mut(&mut caller)[data_ptr as usize .. (data_ptr as usize + data_len as usize)];
                self.handle_write(handle, bytes);
            }),
        ]
    }

    fn write_to_builder(&'static self, mut builder: super::backend::ModuleBuilder) -> super::backend::ModuleBuilder {
        for func in self.functions(&mut builder.store) {
            builder = builder.with_func("env", func.name, func.func);
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
