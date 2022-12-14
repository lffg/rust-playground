use std::sync::atomic::{AtomicU32, Ordering};

static I: AtomicU32 = AtomicU32::new(1);

pub fn next() -> u32 {
    I.fetch_add(1, Ordering::SeqCst)
}
