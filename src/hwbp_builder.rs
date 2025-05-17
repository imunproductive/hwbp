use crate::{BuilderError, Index};
use crate::{Condition, Context, HWBPCallback, HWBPSlot, Size, HWBP};

pub type Result<T> = std::result::Result<T, BuilderError>;

/// A builder for hardware breakpoints.
#[derive(Debug)]
pub struct HWBPBuilder<'a> {
    context: &'a mut Context,
    index: Index,
    is_enabled: bool,
    address: Option<u64>,
    condition: Option<Condition>,
    size: Option<Size>,
    callback: Option<HWBPCallback>,
}

impl<'a> HWBPBuilder<'a> {
    pub(crate) fn new(context: &'a mut Context, index: Index) -> Self {
        Self {
            context,
            index,
            is_enabled: false,
            address: None,
            condition: None,
            size: None,
            callback: None,
        }
    }

    /// Builds and sets the hardware breakpoint.
    pub fn build_and_set(self) -> Result<HWBP> {
        let address = match self.address {
            Some(addr) => addr,
            None => return Err(BuilderError::AddressNotSet),
        };

        let condition = match self.condition {
            Some(condition) => condition,
            None => return Err(BuilderError::ConditionNotSet),
        };

        let callback = match self.callback {
            Some(callback) => callback,
            None => return Err(BuilderError::CallbackNotSet),
        };

        let slot = if condition != Condition::Execute {
            let size = match self.size {
                Some(size) => size,
                None => return Err(BuilderError::SizeNotSet),
            };

            HWBPSlot {
                is_enabled: self.is_enabled,
                address,
                condition,
                size,
            }
        } else {
            HWBPSlot {
                is_enabled: self.is_enabled,
                address,
                condition,
                size: Size::OneByte,
            }
        };

        Ok(self.context.build_and_set_hwbp(self.index, slot, callback))
    }
}

impl HWBPBuilder<'_> {
    /// Watch a memory address for a specific condition.
    pub fn watch_memory(
        mut self,
        addr: *const u8,
        condition: Condition,
        size: Size,
        callback: HWBPCallback,
    ) -> Self {
        self.address = Some(addr as u64);
        self.condition = Some(condition);
        self.size = Some(size);
        self.callback = Some(callback);
        self
    }

    /// Watch a memory address for write access.
    pub fn watch_memory_write(self, addr: *const u8, size: Size, callback: HWBPCallback) -> Self {
        self.watch_memory(addr, Condition::Write, size, callback)
    }

    /// Watch a memory address for read and write access.
    pub fn watch_memory_read_write(
        self,
        addr: *const u8,
        size: Size,
        callback: HWBPCallback,
    ) -> Self {
        self.watch_memory(addr, Condition::ReadWrite, size, callback)
    }

    /// Watch a memory address for execution.
    pub fn watch_memory_execute(self, addr: *const u8, callback: HWBPCallback) -> Self {
        self.watch_memory(addr, Condition::Execute, Size::OneByte, callback)
    }

    /// Watch a variable for a specific condition.
    pub fn watch_variable<T>(
        self,
        variable: &T,
        condition: Condition,
        callback: HWBPCallback,
    ) -> Option<Self> {
        let size = Size::from_bytes(std::mem::size_of::<T>())?;
        Some(self.watch_memory(variable as *const T as *const u8, condition, size, callback))
    }

    /// Watch a variable for write access.
    pub fn watch_variable_write<T>(self, variable: &T, callback: HWBPCallback) -> Option<Self> {
        self.watch_variable(variable, Condition::Write, callback)
    }

    /// Watch a variable for read and write access.
    pub fn watch_variable_read_write<T>(
        self,
        variable: &T,
        callback: HWBPCallback,
    ) -> Option<Self> {
        self.watch_variable(variable, Condition::ReadWrite, callback)
    }
}

impl HWBPBuilder<'_> {
    /// Sets whether the hardware breakpoint is enabled.
    pub fn set_enabled(&mut self, is_enabled: bool) {
        self.is_enabled = is_enabled;
    }

    /// Sets the address of the hardware breakpoint.
    pub fn set_address(&mut self, addr: u64) {
        self.address = Some(addr);
    }

    /// Sets the condition of the hardware breakpoint.
    pub fn set_condition(&mut self, condition: Condition) {
        self.condition = Some(condition);
    }

    /// Sets the size of the hardware breakpoint.
    pub fn set_size(&mut self, size: Size) {
        self.size = Some(size);
    }

    /// Sets the callback of the hardware breakpoint.
    pub fn set_callback(&mut self, callback: HWBPCallback) {
        self.callback = Some(callback);
    }
}

impl HWBPBuilder<'_> {
    /// Sets whether the hardware breakpoint is enabled.
    pub fn with_enabled(mut self, is_enabled: bool) -> Self {
        self.is_enabled = is_enabled;
        self
    }

    /// Sets the address of the hardware breakpoint.
    pub fn with_address(mut self, addr: u64) -> Self {
        self.address = Some(addr);
        self
    }

    /// Sets the condition of the hardware breakpoint.
    pub fn with_condition(mut self, condition: Condition) -> Self {
        self.condition = Some(condition);
        self
    }

    /// Sets the size of the hardware breakpoint.
    pub fn with_size(mut self, size: Size) -> Self {
        self.size = Some(size);
        self
    }

    /// Sets the callback of the hardware breakpoint.
    pub fn with_callback(mut self, callback: HWBPCallback) -> Self {
        self.callback = Some(callback);
        self
    }
}
