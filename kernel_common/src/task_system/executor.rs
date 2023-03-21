use alloc::sync::Arc;
use core::task::{Context, Poll};

use crossbeam_queue::SegQueue;
use futures_task::waker_ref;

use super::task::ArcTask;
use super::spawner::Spawner;

pub type TaskQueue = Arc<SegQueue<ArcTask>>;

pub struct SimpleExecutor {
    task_queue: TaskQueue,
}

impl SimpleExecutor {
    pub fn new() -> SimpleExecutor {
        let task_queue = Arc::new(SegQueue::new());
        SimpleExecutor {
            task_queue,
        }
    }

    pub fn spawner(&self) -> Spawner {
        Spawner::new(self.task_queue.clone())
    }

    fn spawn(&mut self, task: ArcTask) {
        self.task_queue.push(task)
    }

    pub fn task_queue(&self) -> TaskQueue {
        self.task_queue.clone()
    }

    /// Initializes the executor, and never returns
    /// The kernel should simply call this function, and then do everything
    /// afterwards inside async tasks.
    /// ```
    ///
    /// <div class="example-wrap" style="display:inline-block"><pre class="compile_fail" style="white-space:normal;font:inherit;">
    /// **Warning**: This function should not and will not return!
    ///
    /// ```
    pub fn run(self) -> ! {
        self.run_internal()
    }

    fn run_internal(&self) -> ! {
        loop {
            // TODO: This polls tasks until they are ready
            //       For a better implementation, see:
            //       https://os.phil-opp.com/async-await/#executor-with-waker-support
            while let Some(task) = self.task_queue.pop() {
                let mut future_slot = task.future.lock();
                if let Some(mut future) = future_slot.take() {
                    let waker = waker_ref(&task);
                    let mut context = Context::from_waker(&*waker);
                    match future.as_mut().poll(&mut context) {
                        Poll::Ready(()) => {} // Task done
                        Poll::Pending => {
                            *future_slot = Some(future);
                            drop(future_slot);
                        },
                    }
                }
            }
        }
    }
}
