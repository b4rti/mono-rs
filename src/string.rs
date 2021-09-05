use std::ffi::c_void;

use crate::{bindings::MonoString, void_ptr::MonoVoidPtr};

pub struct StringObject {
    pub mono_string: *mut MonoString,
}

impl MonoVoidPtr for StringObject {
    fn as_void_ptr(self) -> *mut c_void {
        self.mono_string as *mut c_void
    }
}
