use crate::types::{Condition, Size};
use bitfield_struct::bitfield;

/// https://en.wikipedia.org/wiki/X86_debug_register#DR6_-_Debug_status
#[bitfield(u64)]
pub struct DR6 {
    pub bp_detected_0: bool,
    pub bp_detected_1: bool,
    pub bp_detected_2: bool,
    pub bp_detected_3: bool,
    #[bits(6)]
    reserved_0: u8,
    blt_exception: bool,
    smm_or_ice_mode: bool, // see DR7 bit 12
    dra_detected: bool,    // see DR7 bit 13
    is_single_step: bool,
    task_switch: bool,
    rtm: bool,
    #[bits(16)]
    reserved_1: u16,
    #[bits(32)]
    reserved_2: u32,
}

/// https://en.wikipedia.org/wiki/X86_debug_register#DR7_-_Debug_control
#[bitfield(u64)]
pub struct DR7 {
    pub bp_local_0: bool,
    pub bp_global_0: bool,
    pub bp_local_1: bool,
    pub bp_global_1: bool,
    pub bp_local_2: bool,
    pub bp_global_2: bool,
    pub bp_local_3: bool,
    pub bp_global_3: bool,
    pub local_exact_bp: bool,
    pub global_exact_bp: bool,
    #[bits(default = true)]
    reserved_0: bool, // always set to 1
    #[bits(default = false)]
    debug_rtm: bool,
    #[bits(default = false)]
    reserved_1: bool,
    pub general_detect: bool,
    #[bits(2)]
    reserved_2: u8,
    #[bits(2)]
    pub bp_condition_0: Condition,
    #[bits(2)]
    pub bp_length_0: Size,
    #[bits(2)]
    pub bp_condition_1: Condition,
    #[bits(2)]
    pub bp_length_1: Size,
    #[bits(2)]
    pub bp_condition_2: Condition,
    #[bits(2)]
    pub bp_length_2: Size,
    #[bits(2)]
    pub bp_condition_3: Condition,
    #[bits(2)]
    pub bp_length_3: Size,
    #[bits(default = false)]
    dr0_pt_log: bool,
    #[bits(default = false)]
    dr1_pt_log: bool,
    #[bits(default = false)]
    dr2_pt_log: bool,
    #[bits(default = false)]
    dr3_pt_log: bool,
    #[bits(28)]
    __: u32,
}
