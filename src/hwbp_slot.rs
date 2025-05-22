use crate::{
    types::{Condition, Index, Size},
    x86::DR7,
};

#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub(crate) struct HWBPSlot {
    pub(crate) is_enabled: bool,
    pub(crate) address: u64,
    pub(crate) condition: Condition,
    pub(crate) size: Size,
}

impl HWBPSlot {
    pub(crate) fn from_dr7(drn: u64, dr7: &DR7, idx: Index) -> Self {
        match idx {
            Index::First => HWBPSlot {
                is_enabled: dr7.bp_local_0(),
                address: drn,
                condition: dr7.bp_condition_0(),
                size: dr7.bp_length_0(),
            },
            Index::Second => HWBPSlot {
                is_enabled: dr7.bp_local_1(),
                address: drn,
                condition: dr7.bp_condition_1(),
                size: dr7.bp_length_1(),
            },
            Index::Third => HWBPSlot {
                is_enabled: dr7.bp_local_2(),
                address: drn,
                condition: dr7.bp_condition_2(),
                size: dr7.bp_length_2(),
            },
            Index::Fourth => HWBPSlot {
                is_enabled: dr7.bp_local_3(),
                address: drn,
                condition: dr7.bp_condition_3(),
                size: dr7.bp_length_3(),
            },
        }
    }

    pub(crate) fn apply_to_dr7(&self, index: &Index, drn: &mut u64, dr7: &mut DR7) {
        *drn = self.address;
        match index {
            Index::First => {
                dr7.set_bp_local_0(self.is_enabled);
                dr7.set_bp_condition_0(self.condition);
                dr7.set_bp_length_0(self.size);
            }
            Index::Second => {
                dr7.set_bp_local_1(self.is_enabled);
                dr7.set_bp_condition_1(self.condition);
                dr7.set_bp_length_1(self.size);
            }
            Index::Third => {
                dr7.set_bp_local_2(self.is_enabled);
                dr7.set_bp_condition_2(self.condition);
                dr7.set_bp_length_2(self.size);
            }
            Index::Fourth => {
                dr7.set_bp_local_3(self.is_enabled);
                dr7.set_bp_condition_3(self.condition);
                dr7.set_bp_length_3(self.size);
            }
        }
    }
}
