use std::ffi::c_void;

use crate::{bindings::MonoArray, AsRawVoid};

pub struct ArrayObject {
    pub mono_array_object: *mut MonoArray,
}

impl AsRawVoid for ArrayObject {
    fn as_raw_void(self) -> *mut c_void {
        self.mono_array_object as *mut c_void
    }
}
