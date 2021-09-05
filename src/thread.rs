use std::ffi::c_void;

use crate::{bindings::MonoThread, void_ptr::MonoVoidPtr};

struct Thread {
    pub mono_thread: *mut MonoThread,
}

impl MonoVoidPtr for Thread {
    fn as_void_ptr(self) -> *mut c_void {
        self.mono_thread as *mut c_void
    }
}
