use alloc::collections::VecDeque;
use alloc::sync::Arc;
use core::task::{Waker, RawWaker, RawWakerVTable, Context, Poll};

use conquer_once::spin::OnceCell;
use crossbeam_queue::SegQueue;
use sync_wrapper::SyncWrapper;
use futures_task::{waker_ref, ArcWake};

use super::task::ArcTask;

pub type TaskQueue = Arc<SegQueue<ArcTask>>;

pub static mut EXECUTOR: SyncWrapper<OnceCell<SimpleExecutor>> = SyncWrapper::new(OnceCell::uninit());

pub fn executor() -> &'static SimpleExecutor {
    unsafe { EXECUTOR.get_mut().get_or_init(|| SimpleExecutor::new()) }
}

pub struct SimpleExecutor {
    task_queue: TaskQueue,
}

impl SimpleExecutor {
    pub fn new() -> SimpleExecutor {
        let task_queue = Arc::new(SegQueue::new());
        info!("Kernel executor initialized!");
        SimpleExecutor {
            task_queue,
        }
    }

    fn spawn(&mut self, task: ArcTask) {
        self.task_queue.push(task)
    }

    pub fn init() {
        let _ = executor();
    }

    pub fn task_queue(&self) -> TaskQueue {
        self.task_queue.clone()
    }

    /// Initializes the executor, and never returns
    /// The kernel should simply call this function, and then do everything
    /// afterwards inside async tasks.
    pub fn run() -> ! {
        executor().run_internal()
    }

    fn run_internal(&self) -> ! {
        loop {
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
                            self.task_queue.push(task.clone());
                        },
                    }
                }
            }
        }
    }
}
