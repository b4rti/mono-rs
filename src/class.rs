use std::ffi::{c_void, CStr, CString};

use crate::{
    bindings::{
        mono_class_get_name, mono_method_desc_new, mono_method_desc_search_in_class,
        mono_object_new, MonoAssembly, MonoClass, MonoDomain, MonoImage,
    },
    method::ClassMethod,
    object::Object,
    AsRawVoid, MonoResult,
};

pub struct Class {
    pub mono_assembly: *mut MonoAssembly,
    pub mono_class: *mut MonoClass,
    pub mono_domain: *mut MonoDomain,
    pub mono_image: *mut MonoImage,
}

impl Class {
    pub fn create_object(&self) -> MonoResult<Object> {
        let mono_object = unsafe { mono_object_new(self.mono_domain, self.mono_class) };

        if mono_object.is_null() {
            return Err("MonoObject Null Error!".into());
        }

        Ok(Object {
            mono_assembly: self.mono_assembly,
            mono_class: self.mono_class,
            mono_domain: self.mono_domain,
            mono_image: self.mono_image,
            mono_object,
        })
    }

    pub fn get_name(&self) -> String {
        unsafe {
            CStr::from_ptr(mono_class_get_name(self.mono_class))
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

        let mono_method =
            unsafe { mono_method_desc_search_in_class(mono_method_desc, self.mono_class) };

        if mono_method.is_null() {
            return Err("MonoMethod Null Error!".into());
        }

        Ok(ClassMethod {
            mono_assembly: self.mono_assembly,
            mono_class: self.mono_class,
            mono_domain: self.mono_domain,
            mono_image: self.mono_image,
            mono_method,
        })
    }
}

impl AsRawVoid for Class {
    fn as_raw_void(self) -> *mut c_void {
        self.mono_class as *mut c_void
    }
}
