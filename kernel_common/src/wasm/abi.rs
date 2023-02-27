use wasmi::core::{HostError, Trap};
use wasmi::ExternRef;

pub trait Abi: Send + Sync {
    fn yield_to_kernel(&self) -> Result<(), Trap> { Err(YieldError.into()) }
    fn int3(&self);

    fn log(&self, data: &[u8]);
}

#[derive(Debug, Copy, Clone)]
pub struct YieldError;

impl core::fmt::Display for YieldError {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        write!(f, "YieldError")
    }
}

impl HostError for YieldError {}
