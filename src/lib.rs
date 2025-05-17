mod context;
mod error;
mod hwbp;
mod hwbp_builder;
mod hwbp_slot;
mod types;
use ::windows::Win32::System::Diagnostics::Debug::EXCEPTION_POINTERS;
pub use context::Context;
pub use error::{BuilderError, ContextError};
pub use hwbp::{HWBPCallback, HWBP};
pub use hwbp_builder::HWBPBuilder;
pub(crate) use hwbp_slot::HWBPSlot;
pub use types::*;

mod callbacks;
mod handler;
mod threads;
mod windows;
mod x86;

/// Initializes the library.
///
/// This method initializes the exception handler.
///
/// If you don't wish this crate to register its own exception handler,
/// and you have your own handler, you should not call this method,
/// and instead call `dispatch_exception`.
pub fn init() {
    handler::init();
}

/// Frees the library.
///
/// This method unregisters the exception handler.
pub fn free() {
    handler::free();
}

/// Frees the library and clears all hardware breakpoints.
pub fn free_and_clear() -> Result<(), ContextError> {
    threads::enumerate(|id| {
        let mut ctx = Context::for_thread(id)?;
        ctx.disable_all();
        ctx.apply_for_thread(id)?;

        Ok(())
    })
    .map_err(|x| match x {
        threads::EnumerateError::WindowsError(e) => ContextError::EnumeratingThreadsFailed(e),
        threads::EnumerateError::UserError(e) => e,
    })?;

    free();
    Ok(())
}

/// Dispatches an exception.
///
/// You should call this method from your exception handler.
/// If you don't have one, you should call `init`.
///
/// # Return value
/// Either EXCEPTION_CONTINUE_EXECUTION or EXCEPTION_CONTINUE_SEARCH.
pub fn dispatch_exception(ex: &mut EXCEPTION_POINTERS) -> i32 {
    unsafe { handler::exception_handler(ex) }
}
