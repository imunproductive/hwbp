use windows::Win32::{
    Foundation::{CloseHandle, HANDLE},
    System::{
        Diagnostics::Debug::{
            GetThreadContext, SetThreadContext, CONTEXT, CONTEXT_DEBUG_REGISTERS_AMD64,
        },
        Threading::{
            GetCurrentThread, GetCurrentThreadId, GetThreadId, OpenThread, THREAD_GET_CONTEXT,
            THREAD_SET_CONTEXT,
        },
    },
};

use crate::{callbacks, threads};
use crate::{windows::AlignedContext, HWBPBuilder};
use crate::{x86::DR7, Index};
use crate::{ContextError, HWBPSlot};
use crate::{HWBPCallback, HWBP};

pub type Result<T> = std::result::Result<T, ContextError>;

/// Represents an X86/AMD64 Windows thread context,
/// but only the hardware breakpoints.
#[derive(Debug, Clone, Copy)]
pub struct Context {
    hwbps: [HWBP; 4],
}

impl Context {
    /// Gets context for the current thread.
    pub fn current() -> Result<Self> {
        let handle = unsafe { GetCurrentThread() };
        let thread_id = unsafe { GetCurrentThreadId() };
        Self::for_handle(handle, Some(thread_id))
    }

    /// Gets context for a specific thread by id.
    pub fn for_thread(thread_id: u32) -> Result<Self> {
        let handle =
            unsafe { OpenThread(THREAD_GET_CONTEXT | THREAD_SET_CONTEXT, false, thread_id) }
                .map_err(ContextError::OpenThreadFailed)?;

        let result = Self::for_handle(handle, Some(thread_id));

        _ = unsafe { CloseHandle(handle) };

        result
    }

    /// Gets context for a specific thread by handle.
    fn for_handle(handle: HANDLE, thread_id: Option<u32>) -> Result<Self> {
        let mut actx = AlignedContext(CONTEXT {
            ContextFlags: CONTEXT_DEBUG_REGISTERS_AMD64,
            ..Default::default()
        });
        unsafe { GetThreadContext(handle, &mut actx.0) }.map_err(ContextError::GetContextFailed)?;

        let thread_id = thread_id.unwrap_or(unsafe { GetThreadId(handle) });

        let ctx = &mut actx.0;
        let dr7 = DR7::from_bits(ctx.Dr7);
        let hwbps: [HWBP; 4] = [
            HWBP::from_context(Index::First, &dr7, ctx.Dr0, thread_id),
            HWBP::from_context(Index::Second, &dr7, ctx.Dr1, thread_id),
            HWBP::from_context(Index::Third, &dr7, ctx.Dr2, thread_id),
            HWBP::from_context(Index::Fourth, &dr7, ctx.Dr3, thread_id),
        ];

        Ok(Self { hwbps })
    }
}

impl Context {
    /// Gets a builder for an unused hardware breakpoint.
    ///
    /// Returns `None` if there are no unused hardware breakpoints.
    #[allow(clippy::manual_map)]
    pub fn unused(&mut self) -> Option<HWBPBuilder> {
        if let Some(hwbp) = self.hwbps.iter().find(|hwbp| !hwbp.is_enabled()) {
            Some(HWBPBuilder::new(self, hwbp.get_index()))
        } else {
            None
        }
    }

    /// Gets the first hardware breakpoint.
    pub fn first(&self) -> HWBP {
        self.hwbps[0]
    }

    /// Gets the second hardware breakpoint.
    pub fn second(&self) -> HWBP {
        self.hwbps[1]
    }

    /// Gets the third hardware breakpoint.
    pub fn third(&self) -> HWBP {
        self.hwbps[2]
    }

    /// Gets the fourth hardware breakpoint.
    pub fn fourth(&self) -> HWBP {
        self.hwbps[3]
    }

    /// Sets a hardware breakpoint.
    pub fn set(&mut self, hwbp: &HWBP) {
        self.hwbps[hwbp.get_index() as usize] = *hwbp;
    }

    /// Disables all hardware breakpoints.
    pub fn disable_all(&mut self) {
        for hwbp in self.hwbps.iter_mut() {
            hwbp.disable();
        }
    }

    pub(crate) fn build_and_set_hwbp(
        &mut self,
        index: Index,
        slot: HWBPSlot,
        callback: HWBPCallback,
    ) -> HWBP {
        let idx = index as usize;
        self.hwbps[idx].set(slot, callback);
        self.hwbps[idx]
    }
}

impl Context {
    /// Applies the context (breakpoints only) to all existing threads.
    pub fn apply_for_all_threads(&self) -> Result<()> {
        threads::enumerate(|id| {
            self.apply_for_thread(id).unwrap();
            Ok(())
        })
        .map_err(|x| match x {
            threads::EnumerateError::WindowsError(e) => ContextError::EnumeratingThreadsFailed(e),
            threads::EnumerateError::UserError(e) => e,
        })?;

        Ok(())
    }

    /// Applies the context (breakpoints only) to the current thread.
    pub fn apply_for_current_thread(&self) -> Result<()> {
        let handle = unsafe { GetCurrentThread() };
        let id = unsafe { GetCurrentThreadId() };

        self.apply_for_handle(handle, Some(id))
    }

    /// Applies the context (breakpoints only) to a specific thread by id.
    pub fn apply_for_thread(&self, thread_id: u32) -> Result<()> {
        let handle =
            unsafe { OpenThread(THREAD_GET_CONTEXT | THREAD_SET_CONTEXT, false, thread_id) }
                .map_err(ContextError::GetContextFailed)?;

        self.apply_for_handle(handle, Some(thread_id))?;

        _ = unsafe { CloseHandle(handle) };
        Ok(())
    }

    /// Applies the context (breakpoints only) to a specific thread by handle.
    fn apply_for_handle(&self, handle: HANDLE, thread_id: Option<u32>) -> Result<()> {
        let mut actx = AlignedContext(CONTEXT {
            ContextFlags: CONTEXT_DEBUG_REGISTERS_AMD64,
            ..Default::default()
        });

        let thread_id = thread_id.unwrap_or(unsafe { GetThreadId(handle) });

        let ctx = &mut actx.0;
        let mut dr7 = DR7::from_bits(ctx.Dr7);
        self.hwbps[0].apply_to_context(&mut ctx.Dr0, &mut dr7);
        self.hwbps[1].apply_to_context(&mut ctx.Dr1, &mut dr7);
        self.hwbps[2].apply_to_context(&mut ctx.Dr2, &mut dr7);
        self.hwbps[3].apply_to_context(&mut ctx.Dr3, &mut dr7);
        ctx.Dr7 = dr7.into_bits();

        let mut write_lock = callbacks::get_write_lock();
        let value = write_lock.entry(thread_id).or_default();
        value[0] = self.hwbps[0].get_callback();
        value[1] = self.hwbps[1].get_callback();
        value[2] = self.hwbps[2].get_callback();
        value[3] = self.hwbps[3].get_callback();

        unsafe { SetThreadContext(handle, ctx) }.map_err(ContextError::SetContextFailed)?;

        Ok(())
    }
}
