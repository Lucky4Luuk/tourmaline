use std::task::Poll;

use crate::abi::{yield_now, poll_promise};

#[derive(Debug)]
pub enum PromiseError {
    PromiseNotFound,
}

pub struct Promise {
    id: i32,
}

impl Promise {
    pub fn new(id: i32) -> Self {
        Self {
            id,
        }
    }

    pub fn poll(&self) -> Result<Poll<i32>, PromiseError> {
        let v = unsafe { poll_promise(self.id) };
        if v == 0 { return Ok(Poll::Pending); }
        if v == 1 { return Err(PromiseError::PromiseNotFound); }
        if v < 0 { return Ok(Poll::Ready(v)); }
        Ok(Poll::Ready(v - 2))
    }
}

pub fn wait_for(promise: Promise) -> i32 {
    loop {
        match promise.poll() {
            Ok(Poll::Pending) => unsafe { yield_now() },
            Ok(Poll::Ready(value)) => return value,
            Err(e) => panic!("Promise returned an error! {:?}", e),
        }
    }
}
