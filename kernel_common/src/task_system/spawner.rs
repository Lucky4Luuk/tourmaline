use core::future::Future;

use super::task::{Task, ArcTask};
use super::executor::{executor, TaskQueue};

#[derive(Clone)]
pub struct Spawner {
    task_queue: TaskQueue,
}

impl Spawner {
    pub fn new() -> Self {
        let task_queue = executor().task_queue();
        Self {
            task_queue,
        }
    }

    pub fn spawn(&self, future: impl Future<Output = ()> + Send + 'static) {
        let task = Task::new(future, self.task_queue.clone());
        self.task_queue.push(task);
    }

    // TODO: Proper async version?
    pub async fn spawn_async(&self, future: impl Future<Output = ()> + Send + 'static) {
        let task = Task::new(future, self.task_queue.clone());
        self.task_queue.push(task);
    }
}
