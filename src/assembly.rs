use std::ffi::c_void;

use crate::{
    bindings::{mono_assembly_get_image, MonoAssembly, MonoDomain},
    image::Image,
    AsRawVoid, MonoResult,
};

pub struct Assembly {
    pub mono_assembly: *mut MonoAssembly,
    pub mono_domain: *mut MonoDomain,
}

impl Assembly {
    pub fn get_image(&self) -> MonoResult<Image> {
        let mono_image = unsafe { mono_assembly_get_image(self.mono_assembly) };

        if mono_image.is_null() {
            return Err("MonoImage Null Error!".into());
        }

        Ok(Image {
            mono_assembly: self.mono_assembly,
            mono_domain: self.mono_domain,
            mono_image,
        })
    }
}

impl AsRawVoid for Assembly {
    fn as_raw_void(self) -> *mut c_void {
        self.mono_assembly as *mut c_void
    }
}
