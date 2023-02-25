use core::future::Future;
use alloc::boxed::Box;

use super::task::Task;
use super::executor::task_queue_push;

#[derive(Copy, Clone)]
pub struct Spawner {

}

impl Spawner {
    pub fn new() -> Self {
        Self {

        }
    }

    pub fn spawn(&self, future: impl Future<Output = ()> + Send + 'static) {
        task_queue_push(Task::new(future));
    }
}
