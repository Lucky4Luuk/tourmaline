use core::{
    future::Future,
    pin::Pin,
    task::{
        Context,
        Poll,
    },
};
use alloc::boxed::Box;

pub struct Task {
    future: Pin<Box< dyn Future<Output = ()> + Send >>,
}

impl Task {
    pub fn new(future: impl Future<Output = ()> + Send + 'static) -> Task {
        Task {
            future: Box::pin(future),
        }
    }

    pub fn poll(&mut self, context: &mut Context) -> Poll<()> {
        self.future.as_mut().poll(context)
    }
}
