use core::future::Future;

use super::task::Task;
use super::executor::TaskQueue;

#[derive(Clone)]
pub struct Spawner {
    task_queue: TaskQueue,
}

impl Spawner {
    pub fn new(task_queue: TaskQueue) -> Self {
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

    /// Returns the amount of tasks currently in the TaskQueue.
    /// Because the TaskQueue is directly shared with the executor, we can know how many
    /// tasks are being executed by the executor!
    pub fn task_count(&self) -> usize {
        self.task_queue.len()
    }
}
