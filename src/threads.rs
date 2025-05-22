use windows::Win32::{
    Foundation::CloseHandle,
    System::Diagnostics::ToolHelp::{
        CreateToolhelp32Snapshot, Thread32First, Thread32Next, TH32CS_SNAPTHREAD, THREADENTRY32,
    },
    System::Threading::GetCurrentProcessId,
};

use thiserror::Error;

#[derive(Error, Debug)]
pub enum EnumerateError<T> {
    WindowsError(#[from] windows::core::Error),
    UserError(T),
}

pub fn enumerate<F, E>(f: F) -> Result<(), EnumerateError<E>>
where
    F: Fn(u32) -> Result<(), E>,
{
    let pid = unsafe { GetCurrentProcessId() };
    let snapshot = unsafe { CreateToolhelp32Snapshot(TH32CS_SNAPTHREAD, 0)? };

    let mut entry = THREADENTRY32 {
        dwSize: std::mem::size_of::<THREADENTRY32>() as u32,
        ..Default::default()
    };

    if unsafe { Thread32First(snapshot, &mut entry) }.is_err() {
        // No threads to enumerate?
        _ = unsafe { CloseHandle(snapshot) };
        return Ok(());
    }

    loop {
        if entry.th32OwnerProcessID == pid {
            f(entry.th32ThreadID).map_err(EnumerateError::UserError)?;
        }

        if unsafe { Thread32Next(snapshot, &mut entry) }.is_err() {
            break;
        }
    }

    _ = unsafe { CloseHandle(snapshot) };
    Ok(())
}
