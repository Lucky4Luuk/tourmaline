pub mod executor;
pub mod task;
pub mod spawner;

use alloc::sync::Arc;

trait ArcWake: Send {
    fn wake_by_ref(arc_self: &Arc<Self>);

    fn wake(self: Arc<Self>) { Self::wake_by_ref(&self) }
}
