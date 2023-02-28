//! The structs/functions defined here are meant to bridge the gap between rust-async and
//! wasm, in a way that's compatible with other languages compiling to wasm as well.

use core::future::Future;
use core::pin::Pin;
use alloc::boxed::Box;

pub type PromiseHandle = u32;

// perhaps use an enum + Into<enum> to fit it into specific return types?
pub struct Promise<'a, T> {
    pub(crate) future: Pin<Box< dyn Future<Output = T> + 'a >>,
}

impl<'a, T> Promise<'a, T> {
    pub fn new(future: Pin<Box< dyn Future<Output = T> + 'a >>) -> Self {
        Self {
            future: future,
        }
    }
}

pub trait AbiPromiseStorage {
    fn store_promise<T>(&self, promise: Promise<T>) -> PromiseHandle;
}

pub trait AbiAsyncBridge: AbiPromiseStorage {
    fn bridge<'a, T>(&self, future: Pin<Box< dyn Future<Output = T> + Send + 'a >>) -> PromiseHandle {
        let promise = Promise::new(future);
        todo!();
    }
}

impl<T: super::abi::Abi + AbiPromiseStorage> AbiAsyncBridge for T {}
