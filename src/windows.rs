use windows::Win32::System::Diagnostics::Debug::CONTEXT;

/// See: https://github.com/microsoft/win32metadata/issues/1044
#[repr(align(16))]
#[derive(Default)]
pub struct AlignedContext(pub CONTEXT);
