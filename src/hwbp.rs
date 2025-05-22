use crate::{callbacks, types::Index, windows::CONTEXT, x86::DR7, HWBPSlot};

/// A callback that is called when the hardware breakpoint is hit.
pub type HWBPCallback = fn(&mut CONTEXT);

/// Represents a hardware breakpoint bound to a specific index.
#[derive(Clone, Copy, Debug)]
pub struct HWBP {
    idx: Index,
    slot: HWBPSlot,
    callback: Option<HWBPCallback>,
}

impl HWBP {
    pub(crate) fn set(&mut self, slot: HWBPSlot, callback: HWBPCallback) {
        self.slot = slot;
        self.callback = Some(callback);
    }

    pub(crate) fn from_context(idx: Index, dr7: &DR7, drn: u64, thread_id: u32) -> Self {
        let slot = HWBPSlot::from_dr7(drn, dr7, idx);
        let callback = callbacks::get(thread_id, idx);
        Self {
            idx,
            slot,
            callback,
        }
    }

    pub(crate) fn apply_to_context(&self, drn: &mut u64, dr7: &mut DR7) {
        self.slot.apply_to_dr7(&self.idx, drn, dr7);
    }
}

impl HWBP {
    /// Gets the index of the hardware breakpoint.
    pub fn get_index(&self) -> Index {
        self.idx
    }

    /// Gets the callback of the hardware breakpoint.
    pub fn get_callback(&self) -> Option<HWBPCallback> {
        self.callback
    }

    /// Gets whether the hardware breakpoint is enabled.
    pub fn is_enabled(&self) -> bool {
        self.slot.is_enabled
    }

    /// Enables the hardware breakpoint.
    pub fn enable(&mut self) {
        self.slot.is_enabled = true;
    }

    /// Disables the hardware breakpoint.
    pub fn disable(&mut self) {
        self.slot.is_enabled = false;
    }
}
