use std::ffi::c_void;

use crate::{bindings::MonoArray, void::AsRawVoid};

#[derive(Clone, Debug)]
pub struct ArrayObject {
    pub mono_ptr: *mut MonoArray,
}

impl AsRawVoid for ArrayObject {
    fn as_raw_void(self) -> *mut c_void {
        self.mono_ptr as *mut c_void
    }
}
