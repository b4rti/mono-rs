use std::ffi::c_void;

use crate::{
    bindings::{mono_assembly_get_image, MonoAssembly, MonoDomain},
    image::Image,
    void_ptr::MonoVoidPtr,
};

pub struct Assembly {
    pub mono_assembly: *mut MonoAssembly,
    pub mono_domain: *mut MonoDomain,
}

impl Assembly {
    pub fn get_image(&self) -> Image {
        Image {
            mono_assembly: self.mono_assembly,
            mono_domain: self.mono_domain,
            mono_image: unsafe { mono_assembly_get_image(self.mono_assembly) },
        }
    }
}

impl MonoVoidPtr for Assembly {
    fn as_void_ptr(self) -> *mut c_void {
        self.mono_assembly as *mut c_void
    }
}
