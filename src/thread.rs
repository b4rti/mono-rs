use std::ffi::c_void;

use crate::{bindings::MonoThread, void::AsRawVoid};

#[derive(Clone, Debug)]
struct Thread {
    pub mono_ptr: *mut MonoThread,
}

impl AsRawVoid for Thread {
    fn as_raw_void(self) -> *mut c_void {
        self.mono_ptr as *mut c_void
    }
}
