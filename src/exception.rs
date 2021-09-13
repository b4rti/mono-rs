use std::ffi::c_void;

use crate::{bindings::MonoException, AsRawVoid};

struct Exception {
    pub mono_exception: *mut MonoException,
}

impl AsRawVoid for Exception {
    fn as_raw_void(self) -> *mut c_void {
        self.mono_exception as *mut c_void
    }
}
