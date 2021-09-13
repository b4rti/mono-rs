use std::ffi::{c_void, CString};

use crate::{
    bindings::{mono_class_from_name, MonoAssembly, MonoDomain, MonoImage},
    class::Class,
    AsRawVoid, MonoResult,
};

pub struct Image {
    pub mono_assembly: *mut MonoAssembly,
    pub mono_domain: *mut MonoDomain,
    pub mono_image: *mut MonoImage,
}

impl Image {
    pub fn get_class_by_name<T: AsRef<str>>(&self, name_space: T, name: T) -> MonoResult<Class> {
        let class_name = CString::new(name.as_ref())?;
        let class_name_space = CString::new(name_space.as_ref())?;
        let mono_class = unsafe {
            mono_class_from_name(
                self.mono_image,
                class_name_space.as_ptr(),
                class_name.as_ptr(),
            )
        };

        if mono_class.is_null() {
            return Err("MonoClass Null Error!".into());
        }

        Ok(Class {
            mono_assembly: self.mono_assembly,
            mono_class,
            mono_domain: self.mono_domain,
            mono_image: self.mono_image,
        })
    }
}

impl AsRawVoid for Image {
    fn as_raw_void(self) -> *mut c_void {
        self.mono_image as *mut c_void
    }
}
