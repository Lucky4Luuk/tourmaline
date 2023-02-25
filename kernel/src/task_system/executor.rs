use alloc::collections::VecDeque;
use core::task::{Waker, RawWaker, RawWakerVTable, Context, Poll};

use super::task::Task;
use super::spawner::Spawner;

use lazy_static::lazy_static;
use conquer_once::spin::OnceCell;
use crossbeam_queue::SegQueue;

pub type TaskQueue = SegQueue<Task>;

lazy_static! {
    static ref TASK_QUEUE: OnceCell<TaskQueue> = OnceCell::uninit();
}

fn task_queue() -> &'static TaskQueue {
    TASK_QUEUE.get_or_init(|| TaskQueue::new())
}

pub fn task_queue_push(task: Task) {
    task_queue().push(task);
}

pub async fn task_queue_push_async(task: Task) {
    task_queue().push(task);
}

fn task_queue_pop() -> Option<Task> {
    task_queue().pop()
}

pub struct SimpleExecutor {
    task_queue: VecDeque<Task>,
}

impl SimpleExecutor {
    fn new() -> SimpleExecutor {
        info!("Kernel executor initialized!");
        SimpleExecutor {
            task_queue: VecDeque::new(),
        }
    }

    fn spawn(&mut self, task: Task) {
        self.task_queue.push_back(task)
    }

    /// Initializes the executor, and never returns
    /// The kernel should simply call this function, and then do everything
    /// afterwards inside async tasks.
    pub fn run() -> ! {
        Self::new().run_internal()
    }

    fn run_internal(mut self) -> ! {
        loop {
            while let Some(task) = task_queue_pop() {
                self.spawn(task);
            }

            while let Some(mut task) = self.task_queue.pop_front() {
                let waker = dummy_waker();
                let mut context = Context::from_waker(&waker);
                match task.poll(&mut context) {
                    Poll::Ready(()) => {} // task done
                    Poll::Pending => self.task_queue.push_back(task),
                }
            }
        }
    }
}

fn dummy_raw_waker() -> RawWaker {
    fn no_op(_: *const ()) {}
    fn clone(_: *const ()) -> RawWaker {
        dummy_raw_waker()
    }

    let vtable = &RawWakerVTable::new(clone, no_op, no_op, no_op);
    RawWaker::new(0 as *const (), vtable)
}

fn dummy_waker() -> Waker {
    unsafe { Waker::from_raw(dummy_raw_waker()) }
}
