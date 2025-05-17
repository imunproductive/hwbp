use lazy_static::lazy_static;
use std::{
    collections::HashMap,
    sync::{RwLock, RwLockWriteGuard},
};

use crate::{HWBPCallback, Index};

type HWBPCallbackList = [Option<HWBPCallback>; 4];

lazy_static! {
    static ref CALLBACKS: RwLock<HashMap<u32, HWBPCallbackList>> = RwLock::new(HashMap::new());
}

pub fn get(thread_id: u32, index: Index) -> Option<HWBPCallback> {
    if let Ok(callbacks) = CALLBACKS.read() {
        callbacks
            .get(&thread_id)
            .and_then(|callback_list| callback_list.get(index as usize))
            .and_then(|x| *x)
    } else {
        None
    }
}

pub fn get_write_lock() -> RwLockWriteGuard<'static, HashMap<u32, HWBPCallbackList>> {
    CALLBACKS
        .write()
        .expect("Failed to acquire write lock for callbacks")
}
