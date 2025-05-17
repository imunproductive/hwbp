use thiserror::Error;
use windows::core::Error as WindowsError;

#[derive(Error, Debug)]
pub enum ContextError {
    #[error("Failed to open thread: {0}")]
    OpenThreadFailed(WindowsError),
    #[error("Failed to get context: {0}")]
    GetContextFailed(WindowsError),
    #[error("Failed to set context: {0}")]
    SetContextFailed(WindowsError),
    #[error("Error enumerating threads: {0}")]
    EnumeratingThreadsFailed(WindowsError),
}

#[derive(Error, Debug)]
pub enum BuilderError {
    #[error("Adddress is not set")]
    AddressNotSet,
    #[error("Condition is not set")]
    ConditionNotSet,
    #[error("Size is not set")]
    SizeNotSet,
    #[error("Callback is not set")]
    CallbackNotSet,
}
