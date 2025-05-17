# HWBP

A fully-featured Rust library for managing hardware breakpoints on Windows via [x86 debug registers](https://en.wikipedia.org/wiki/X86_debug_register).

HWBP provides a clean API to set, manage, and handle hardware breakpoints for watching memory execution, read & write access.

## Limitations

All of the following limitations are due to the hardware limitations of x86/AMD64 architecture.

- **Maximum of 4 hardware breakpoints**: There are only 4 debug registers available for breakpoints (DR0-DR3).
- **Thread-specific**: Breakpoints can only be applied to existing threads, not to threads created after setting the breakpoint. [^1]
- **Size restrictions**: Breakpoints can only monitor 1, 2, 4, or 8 bytes of memory, depending on the architecture.

## Usage

First things first, you need to initialize the library:

```rust
hwbp::init();
```

This will intiialzie the exception handler. However, if you have one already, you can call `dispatch_exception` instead.

Now you need to obtain `Context` from current thread:

```rust
let mut ctx = Context::current().unwrap();
```

You can also obtain `Context` from a specific thread:

```rust
let mut ctx = Context::for_thread(42).unwrap();
```

Once you have a `Context`, you can create a new `HWBP` using `Context::unused` method:

```rust
let mut x = 0;
let hwbp = ctx
    .unused()
    .unwrap()
    .watch_variable_write(&x, |_| {
        println!("callback!")
    })
    .unwrap()
    .with_enabled(true)
    .build()
    .unwrap();
```

You can also make up any `HWBP` you want:

```rust
let mut hwbp = ctx
    .unused()
    .unwrap()
    .with_enabled(true)
    .with_address(0x12345678)
    .with_condition(Condition::ReadWrite)
    .with_size(Size::EightBytes)
    .with_callback(|_| {
        println!("callback!")
    })
    .build_and_set()
    .unwrap();
```

This will create a new `HWBP` and set it to context, however, not yet applied to the current thread.

```rust
ctx.apply_for_current_thread().expect("Failed to apply");
```

Voila!

For more examples, check out the [examples](./examples/) directory!

To free the library you just call `free`:

```rust
hwbp::free();
```
