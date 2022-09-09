use std::ffi::c_void;

use crate::{bindings::MonoException, void::AsRawVoid};

#[derive(Clone, Debug)]
struct Exception {
    pub mono_ptr: *mut MonoException,
}

impl AsRawVoid for Exception {
    fn as_raw_void(self) -> *mut c_void {
        self.mono_ptr as *mut c_void
    }
}
