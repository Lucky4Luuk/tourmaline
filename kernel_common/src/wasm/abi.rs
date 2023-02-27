use wasmi::core::{HostError, Trap};

pub trait Abi: Send + Sync {
    fn yield_to_kernel(&self) -> Result<(), Trap> { Err(YieldError(0).into()) }
    fn int3(&self);
}

#[derive(Debug, Copy, Clone)]
pub struct YieldError(u32);

impl core::fmt::Display for YieldError {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        write!(f, "YieldError")
    }
}

impl HostError for YieldError {}
