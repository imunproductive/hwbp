use hwbp::Context;
use std::sync::atomic::AtomicU32;

static TRIGGERED: AtomicU32 = AtomicU32::new(0);

pub fn main() {
    hwbp::init();

    let mut x = 42;

    let mut ctx = Context::current().unwrap();
    let mut hwbp = ctx
        .unused()
        .unwrap()
        .watch_variable_write(&x, |_| {
            TRIGGERED.fetch_add(1, std::sync::atomic::Ordering::Relaxed);
            println!("callback")
        })
        .unwrap()
        .with_enabled(true)
        .build_and_set()
        .unwrap();
    ctx.apply_for_current_thread().expect("Failed to apply");

    unsafe { core::ptr::write_volatile(&mut x, 69) };

    hwbp.disable();
    ctx.set(&hwbp);
    ctx.apply_for_current_thread().expect("Failed to apply");

    println!("x = {}", x);
    println!(
        "triggered = {}",
        TRIGGERED.load(std::sync::atomic::Ordering::Relaxed)
    );

    hwbp::free();
}
