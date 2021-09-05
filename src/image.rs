use std::{
    ffi::{c_void, CString},
    sync::Arc,
};

use crate::{
    assembly::Assembly,
    bindings::{mono_class_from_name, MonoImage},
    class::Class,
    domain::Domain,
    void::AsRawVoid,
    MonoResult,
};

#[derive(Clone, Debug)]
pub struct Image {
    pub mono_ptr: *mut MonoImage,
    pub assembly: Arc<Assembly>,
    pub domain: Arc<Domain>,
}

impl Image {
    pub fn get_class_by_name<T: AsRef<str>>(&self, name_space: T, name: T) -> MonoResult<Class> {
        let name = CString::new(name.as_ref())?;
        let name_space = CString::new(name_space.as_ref())?;
        let mono_ptr =
            unsafe { mono_class_from_name(self.mono_ptr, name_space.as_ptr(), name.as_ptr()) };

        if mono_ptr.is_null() {
            return Err("MonoClass Null Error!".into());
        }

        Ok(Class {
            mono_ptr,
            assembly: self.assembly.clone(),
            domain: self.domain.clone(),
            image: Arc::new(self.clone()),
        })
    }
}

impl AsRawVoid for Image {
    fn as_raw_void(self) -> *mut c_void {
        self.mono_ptr as *mut c_void
    }
}
