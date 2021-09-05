use std::ffi::c_void;

use crate::{bindings::MonoArray, void_ptr::MonoVoidPtr};

pub struct ArrayObject {
    pub mono_array_object: *mut MonoArray,
}

impl MonoVoidPtr for ArrayObject {
    fn mono_void_ptr(self) -> *mut c_void {
        self.mono_array_object as *mut c_void
    }
}
