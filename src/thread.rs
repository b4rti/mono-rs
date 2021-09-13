use std::ffi::c_void;

use crate::{bindings::MonoThread, AsRawVoid};

struct Thread {
    pub mono_thread: *mut MonoThread,
}

impl AsRawVoid for Thread {
    fn as_raw_void(self) -> *mut c_void {
        self.mono_thread as *mut c_void
    }
}
