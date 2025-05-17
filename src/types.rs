/// The index of a hardware breakpoint.
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
pub enum Index {
    First = 0,
    Second = 1,
    Third = 2,
    Fourth = 3,
}

/// The size of a hardware breakpoint.
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[repr(u8)]
pub enum Size {
    OneByte,
    TwoBytes,
    FourBytes,
    #[cfg(target_arch = "x86_64")]
    EightBytes,
}

impl Size {
    pub(crate) const fn into_bits(self) -> u8 {
        self as _
    }
    pub(crate) const fn from_bits(value: u8) -> Self {
        match value {
            0 => Self::OneByte,
            1 => Self::TwoBytes,
            2 => Self::FourBytes,
            #[cfg(target_arch = "x86_64")]
            3 => Self::EightBytes,
            _ => unreachable!(),
        }
    }

    pub(crate) const fn from_bytes(value: usize) -> Option<Self> {
        match value {
            1 => Some(Self::OneByte),
            2 => Some(Self::TwoBytes),
            4 => Some(Self::FourBytes),
            #[cfg(target_arch = "x86_64")]
            8 => Some(Self::EightBytes),
            _ => None,
        }
    }
}

/// The condition of a hardware breakpoint.
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
#[repr(u8)]
pub enum Condition {
    Execute = 0b00,
    Write = 0b01,
    ReadWrite = 0b11,
    /// You most definitely don't want to use this one.
    IoReadWrite = 0b10,
}

impl Condition {
    pub(crate) const fn into_bits(self) -> u8 {
        self as _
    }

    pub(crate) const fn from_bits(value: u8) -> Self {
        match value {
            0b00 => Self::Execute,
            0b01 => Self::Write,
            0b11 => Self::ReadWrite,
            0b10 => Self::IoReadWrite,
            _ => unreachable!(),
        }
    }
}
