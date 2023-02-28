use core::{
    future::Future,
    pin::Pin,
    task::{
        Context,
        Poll,
    },
};
use alloc::{
    boxed::Box,
    sync::Arc,
};

use futures_task::ArcWake;
use super::executor::TaskQueue;

pub type ArcTask = Arc<Task>;

pub struct Task {
    pub(crate) future: spin::Mutex<Option<Pin<Box< dyn Future<Output = ()> + Send >>>>,
    task_queue: TaskQueue,
}

impl Task {
    pub fn new(future: impl Future<Output = ()> + Send + 'static, task_queue: TaskQueue) -> ArcTask {
        Arc::new(Self {
            future: spin::Mutex::new(Some(Box::pin(future))),
            task_queue,
        })
    }
}

impl ArcWake for Task {
    fn wake_by_ref(arc_self: &Arc<Self>) {
        let cloned = arc_self.clone();
        arc_self.task_queue.push(cloned);
    }
}

#[inline]
pub async fn yield_now() {
    YieldNow(false).await
}

struct YieldNow(bool);

impl Future for YieldNow {
    type Output = ();
    fn poll(mut self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Self::Output> {
        if !self.0 {
            self.0 = true;
            cx.waker().wake_by_ref();
            Poll::Pending
        } else {
            Poll::Ready(())
        }
    }
}
