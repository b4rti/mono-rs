use std::{
    ffi::{c_void, CStr, CString},
    sync::Arc,
};

use crate::{
    assembly::Assembly,
    bindings::{
        mono_class_get_name, mono_method_desc_new, mono_method_desc_search_in_class,
        mono_object_new, MonoClass,
    },
    domain::Domain,
    image::Image,
    method::ClassMethod,
    object::Object,
    void::AsRawVoid,
    MonoResult,
};

#[derive(Clone, Debug)]
pub struct Class {
    pub mono_ptr: *mut MonoClass,
    pub assembly: Arc<Assembly>,
    pub domain: Arc<Domain>,
    pub image: Arc<Image>,
}

impl Class {
    pub fn create_object(&self) -> MonoResult<Object> {
        let mono_ptr = unsafe { mono_object_new(self.domain.mono_ptr, self.mono_ptr) };

        if mono_ptr.is_null() {
            return Err("MonoObject Null Error!".into());
        }

        Ok(Object {
            mono_ptr,
            assembly: self.assembly.clone(),
            class: Arc::new(self.clone()),
            domain: self.domain.clone(),
            image: self.image.clone(),
        })
    }

    pub fn get_name(&self) -> String {
        unsafe {
            CStr::from_ptr(mono_class_get_name(self.mono_ptr))
                .to_string_lossy()
                .to_string()
        }
    }

    pub fn get_method_by_name(&self, name: &'static str) -> MonoResult<ClassMethod> {
        let class_name = self.get_name();
        let method_name = CString::new(format!("{}:{}()", class_name, name))?;
        let mono_method_desc = unsafe { mono_method_desc_new(method_name.as_ptr(), 0) };

        if mono_method_desc.is_null() {
            return Err("MonoMethodDecs Null Error!".into());
        }

        let mono_ptr =
            unsafe { mono_method_desc_search_in_class(mono_method_desc, self.mono_ptr) };

        if mono_ptr.is_null() {
            return Err("MonoMethod Null Error!".into());
        }

        Ok(ClassMethod {
            mono_ptr,
            assembly: self.assembly.clone(),
            class: Arc::new(self.clone()),
            domain: self.domain.clone(),
            image: self.image.clone(),
        })
    }
}

impl AsRawVoid for Class {
    fn as_raw_void(self) -> *mut c_void {
        self.mono_ptr as *mut c_void
    }
}
