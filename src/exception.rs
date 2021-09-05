use std::ffi::c_void;

use crate::{bindings::MonoException, void_ptr::MonoVoidPtr};

struct Exception {
    pub mono_exception: *mut MonoException,
}

impl MonoVoidPtr for Exception {
    fn to_mono_ptr(self) -> *mut c_void {
        self.mono_exception as *mut c_void
    }
}
