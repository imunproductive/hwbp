use std::sync::Mutex;

use windows::Win32::{
    Foundation::EXCEPTION_SINGLE_STEP,
    System::{
        Diagnostics::Debug::{
            AddVectoredExceptionHandler, RemoveVectoredExceptionHandler,
            EXCEPTION_CONTINUE_EXECUTION, EXCEPTION_CONTINUE_SEARCH, EXCEPTION_POINTERS,
        },
        Threading::GetCurrentThreadId,
    },
};

use crate::{
    callbacks,
    x86::{DR6, DR7},
    Index,
};

static HANDLER_HANDLE: Mutex<Option<usize>> = Mutex::new(None);

pub fn init() {
    let mut lock = HANDLER_HANDLE.lock().unwrap();
    if lock.is_some() {
        return;
    }

    let handler = unsafe { AddVectoredExceptionHandler(1, Some(exception_handler)) };
    *lock = Some(handler as usize);
}

pub fn free() {
    let mut lock = HANDLER_HANDLE.lock().unwrap();

    if let Some(handler) = lock.take() {
        unsafe { RemoveVectoredExceptionHandler(handler as _) };
    }
}

pub unsafe extern "system" fn exception_handler(ex: *mut EXCEPTION_POINTERS) -> i32 {
    if let Some(ex) = ex.as_mut() {
        let cr = ex.ContextRecord;
        let er = ex.ExceptionRecord;
        if let (Some(cr), Some(er)) = (cr.as_mut(), er.as_ref()) {
            if er.ExceptionCode == EXCEPTION_SINGLE_STEP {
                let mut dr6 = DR6::from_bits(cr.Dr6);
                let dr7 = DR7::from_bits(cr.Dr7);
                let tid = GetCurrentThreadId();

                if dr7.bp_local_0() && dr6.bp_detected_0() {
                    if let Some(callback) = callbacks::get(tid, Index::First) {
                        callback(cr);
                    }
                    dr6.set_bp_detected_0(false);
                } else if dr7.bp_local_1() && dr6.bp_detected_1() {
                    if let Some(callback) = callbacks::get(tid, Index::Second) {
                        callback(cr);
                    }
                    dr6.set_bp_detected_1(false);
                } else if dr7.bp_local_2() && dr6.bp_detected_2() {
                    if let Some(callback) = callbacks::get(tid, Index::Third) {
                        callback(cr);
                    }
                    dr6.set_bp_detected_2(false);
                } else if dr7.bp_local_3() && dr6.bp_detected_3() {
                    if let Some(callback) = callbacks::get(tid, Index::Fourth) {
                        callback(cr);
                    }
                    dr6.set_bp_detected_3(false);
                }

                cr.Dr6 = dr6.into_bits();
                cr.EFlags |= 1 << 16;
                return EXCEPTION_CONTINUE_EXECUTION;
            }
        }
    }

    EXCEPTION_CONTINUE_SEARCH
}
