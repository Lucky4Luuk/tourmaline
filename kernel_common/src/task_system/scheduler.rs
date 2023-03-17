use alloc::string::String;
use alloc::boxed::Box;
use alloc::sync::Arc;
use alloc::vec::Vec;

use core::future::Future;
use core::pin::Pin;

use sync_wrapper::SyncWrapper;

use super::spawner::Spawner;
use crate::services::*;

pub struct SchedulerMessage {
    future: spin::Mutex<SyncWrapper<Option<Pin<Box< dyn Future<Output = ()> + Send >>>>>,
}

impl Message for SchedulerMessage {
    fn target(&self) -> &str { "scheduler_service" }
}

pub struct SchedulerService;
impl Service for SchedulerService {
    fn name(&self) -> String { String::from("scheduler_service") }
    fn push_message(&self, message: ArcMessage) {
        if let Some(msg) = message.as_any().downcast_ref::<SchedulerMessage>() {
            if let Some(task) = msg.future.lock().get_mut().take() {
                scheduler_spawn_task(task);
            }
        }
    }
}

pub(crate) static SCHEDULER: spin::Mutex<Scheduler> = spin::Mutex::new(Scheduler::new());

/// Generic function for modifying the global scheduler.
pub fn with_scheduler<T, F: FnOnce(&mut Scheduler) -> T>(f: F) -> T {
    let mut lock = SCHEDULER.lock();
    let res = f(&mut lock);
    drop(lock);
    res
}

pub fn scheduler_spawn_task(task: impl core::future::Future<Output = ()> + Send + 'static) -> usize {
    with_scheduler(|sched| sched.spawn_task(task))
}

/// Spawns an async task on the global scheduler.
/// See `[Scheduler::spawn_task]` for more information.
pub fn scheduler_add_spawner(spawner: Spawner) {
    with_scheduler(|sched| sched.add_spawner(spawner));
}

/// The global scheduler implementation. This will ensure that tasks
/// end up (hopefully) on under utilized cores/executors.
/// Currently, it's very basic, and just offloads new tasks to
/// the executor with the least amount of tasks running on it already.
pub struct Scheduler {
    spawners: Vec<Spawner>,
}

impl Scheduler {
    pub const fn new() -> Self {
        Self {
            spawners: Vec::new(),
        }
    }

    /// Add a Spawner to the scheduler. If you add 2 spawners both referencing
    /// the same executor, it just makes spawning tasks slower.
    /// In the future, with a different method to select an executor, this might
    /// become undefined behaviour!
    pub fn add_spawner(&mut self, spawner: Spawner) {
        self.spawners.push(spawner);
    }

    /// Spawns an async task. Automatically tries to pick the best spawner
    /// to spawn the task on.
    /// Panics if there are no spawners in the scheduler.
    pub fn spawn_task(&self, task: impl core::future::Future<Output = ()> + Send + 'static) -> usize {
        let mut lowest_idx = 0;
        let mut lowest_count = usize::MAX;
        for (i, spawner) in self.spawners.iter().enumerate() {
            let task_count = spawner.task_count();
            if task_count < lowest_count {
                lowest_idx = i;
                lowest_count = task_count;
            }
        }
        self.spawners[lowest_idx].spawn(task);
        lowest_idx
    }
}
