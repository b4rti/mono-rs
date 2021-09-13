use std::ffi::c_void;

use crate::{bindings::MonoString, AsRawVoid};

pub struct StringObject {
    pub mono_string: *mut MonoString,
}

impl AsRawVoid for StringObject {
    fn as_raw_void(self) -> *mut c_void {
        self.mono_string as *mut c_void
    }
}
