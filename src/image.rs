use std::ffi::{c_void, CString};

use crate::{
    bindings::{mono_class_from_name, MonoAssembly, MonoDomain, MonoImage},
    class::Class,
    void_ptr::MonoVoidPtr,
};

pub struct Image {
    pub mono_assembly: *mut MonoAssembly,
    pub mono_domain: *mut MonoDomain,
    pub mono_image: *mut MonoImage,
}

impl Image {
    pub fn get_class_by_name(&self, name_space: &'static str, name: &'static str) -> Class {
        let class_name = CString::new(name).unwrap();
        let class_name_space = CString::new(name_space).unwrap();
        let class = unsafe {
            mono_class_from_name(
                self.mono_image,
                class_name_space.as_ptr(),
                class_name.as_ptr(),
            )
        };
        Class {
            mono_assembly: self.mono_assembly,
            mono_class: class,
            mono_domain: self.mono_domain,
            mono_image: self.mono_image,
        }
    }
}

impl MonoVoidPtr for Image {
    fn as_void_ptr(self) -> *mut c_void {
        self.mono_image as *mut c_void
    }
}
