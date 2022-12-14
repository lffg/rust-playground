use std::rc::Rc;

use tokio::{join, task, task_local};
use tracing::{instrument, trace, Instrument, Span};

mod assert;
mod seq;
mod setup;
mod task_id;

task_local! {
    static COUNTER: i32;
}

#[tokio::main]
async fn main() {
    setup::setup_tracing();

    let a = fut("a");
    trace!("created fut `a`");
    let b = fut("b");
    trace!("created fut `b`");

    join!(a, b);
    trace!("finished");
}

#[instrument]
async fn fut(label: &'static str) {
    // Obviously, in order to `get` the current `task_id`, one needs to define a
    // task scope.
    task_id::scope(async {
        trace!("just in");
        let id = task_id::get();
        trace!(id, "got this id");

        // This `Future` is `!Send` now.
        let rc = Rc::new(());

        let handle = tokio::spawn(
            // When spawning a new task, one needs to create a new scope, since
            // a new task is being created. Hence, the underlying task local
            // starts without a value.
            task_id::scope(async {
                other().await;
            })
            .instrument(Span::current()),
        );
        handle.await.unwrap();

        use_rc(rc.clone());
    })
    .await
}

#[instrument]
async fn other() {
    trace!("just in");
    task::yield_now().await;
    let id = task_id::get();
    trace!(id, "i'm inside the `other` future");
}

fn use_rc(_rc: Rc<()>) {
    // Do stuff w/ rc
}
