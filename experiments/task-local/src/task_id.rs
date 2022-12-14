use std::future::Future;

use tokio::task_local;
use tracing::trace;

use crate::seq;

task_local! {
    static ID: TaskId;
}

/// Creates a new [TaskId] scope.
pub async fn scope<F, R>(fut: F) -> R
where
    F: Future<Output = R>,
{
    ID.scope(TaskId::new(), fut).await
}

/// Returns the associated task id.
///
/// # Panics
///
/// Panics if the current task is not running within a valid scope.
pub fn get() -> u32 {
    ID.with(TaskId::id)
}

struct TaskId(u32);

impl TaskId {
    fn new() -> Self {
        let id = seq::next();
        trace!(id, "created TaskId");
        Self(id)
    }

    fn id(&self) -> u32 {
        self.0
    }
}

impl Drop for TaskId {
    fn drop(&mut self) {
        trace!(id = self.id(), "dropped TaskId");
    }
}
