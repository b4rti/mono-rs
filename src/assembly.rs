use std::{ffi::c_void, sync::Arc};

use crate::{
    bindings::{mono_assembly_get_image, MonoAssembly},
    domain::Domain,
    image::Image,
    void::AsRawVoid,
    MonoResult,
};

#[derive(Clone, Debug)]
pub struct Assembly {
    pub mono_ptr: *mut MonoAssembly,
    pub domain: Arc<Domain>,
}

impl Assembly {
    pub fn get_image(&self) -> MonoResult<Image> {
        let mono_ptr = unsafe { mono_assembly_get_image(self.mono_ptr) };

        if mono_ptr.is_null() {
            return Err("MonoImage Null Error!".into());
        }

        Ok(Image {
            mono_ptr,
            assembly: Arc::new(self.clone()),
            domain: self.domain.clone(),
        })
    }
}

impl AsRawVoid for Assembly {
    fn as_raw_void(self) -> *mut c_void {
        self.mono_ptr as *mut c_void
    }
}
