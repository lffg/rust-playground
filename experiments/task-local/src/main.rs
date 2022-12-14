use std::{
    future::{self, Future},
    rc::Rc,
    task::Poll,
};

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

    // (ref:1)
    let a = fut("a");
    trace!("created fut `a`");

    // Obviously, in order to `get` the current `task_id`, one needs to define a
    // task scope.
    task_id::scope(async {
        // (ref:1)
        // Scoping only matters when the future is polled.
        let b = fut("b");
        trace!("created fut `b`");

        join!(a, b);
        trace!("finished `a` and `b`");

        // Below we demonstrate that a Future may be moved from tasks across an
        // `.await` point.
        let mut fut = Box::pin(another());
        future::poll_fn(|cx| {
            let res = fut.as_mut().poll(cx);
            assert!(res.is_pending());
            Poll::Ready(())
        })
        .await;
        tokio::spawn(task_id::scope(async {
            fut.await;
        }))
        .await
        .unwrap();
    })
    .await;
}

#[instrument]
async fn fut(label: &'static str) {
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

    noop(rc.clone());
}

#[instrument]
async fn other() {
    trace!("just in");
    task::yield_now().await;
    let id = task_id::get();
    trace!(id, "i'm inside the `other` future");
}

#[instrument]
async fn another() {
    let id = task_id::get();
    trace!(id, "will yield");
    task::yield_now().await;

    let id = task_id::get();
    trace!(id, "again here");
}

fn noop<T>(_val: T) {}
