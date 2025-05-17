use hwbp::Context;
use windows::Win32::System::Diagnostics::Debug::CONTEXT;

#[inline(never)]
extern "system" fn test_method(num: u32) {
    println!("test_method called with {}", num);
}

fn hooked_method(ctx: &mut CONTEXT) {
    println!("hooked_method called");
    ctx.Rcx += 27;
}

fn main() {
    hwbp::init();

    let mut ctx = Context::current().unwrap();
    let mut hwbp = ctx
        .unused()
        .unwrap()
        .watch_memory_execute(test_method as _, hooked_method)
        .with_enabled(true)
        .build_and_set()
        .unwrap();
    ctx.apply_for_current_thread().expect("Failed to apply");

    test_method(42);

    hwbp.disable();
    ctx.set(&hwbp);
    ctx.apply_for_current_thread().expect("Failed to apply");

    test_method(42);

    // Output:
    // hooked_method called
    // test_method called with 69
    // test_method called with 42

    hwbp::free();
}
