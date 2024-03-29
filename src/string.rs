use std::ffi::c_void;

use crate::{bindings::MonoString, void::AsRawVoid};

#[derive(Clone, Debug)]
pub struct StringObject {
    pub mono_ptr: *mut MonoString,
}

impl AsRawVoid for StringObject {
    fn as_raw_void(self) -> *mut c_void {
        self.mono_ptr as *mut c_void
    }
}
