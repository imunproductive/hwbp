pub use windows::{
    core::Error,
    Win32::System::Diagnostics::Debug::{CONTEXT, CONTEXT_FLAGS, EXCEPTION_POINTERS},
};

/// See: https://github.com/microsoft/win32metadata/issues/1044
#[repr(align(16))]
#[derive(Default)]
pub(crate) struct AlignedContext(pub CONTEXT);
