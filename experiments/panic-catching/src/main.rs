use std::{
    any::Any,
    panic::{self, PanicInfo, UnwindSafe},
};

fn d() {
    println!("will call panic! now:");
    panic!("panic msg");
}

fn c() {
    d();
}

fn b() {
    c();
}

fn a() {
    b();
}

fn main() {
    install_hook();

    catcher(|| {
        a();
    });
}

fn downcast_str(payload: &dyn Any) -> Option<&str> {
    payload
        .downcast_ref::<String>()
        .map(|s| s.as_str())
        .or_else(|| payload.downcast_ref::<&str>().copied())
}

fn catcher(f: impl FnOnce() + UnwindSafe) {
    let result = panic::catch_unwind(f);
    if let Err(payload) = result {
        let message = downcast_str(&payload);
        dbg!(message);
    }
}

fn install_hook() {
    // when a panic occurs, rust will first invoke the panic hook and then
    // invoke the unwinding runtime (or just abort, if it's the case).
    panic::set_hook(Box::new(panic_hook));
}

fn panic_hook(info: &PanicInfo) {
    println!("thread panicked!");
    dbg!(info);
}
