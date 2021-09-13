use std::{
    ffi::{c_void, CStr, CString},
    ptr::null_mut,
};

use crate::{
    bindings::{
        mono_class_get_field_from_name, mono_class_get_name, mono_method_desc_new,
        mono_method_desc_search_in_class, mono_runtime_invoke, MonoAssembly, MonoClass, MonoDomain,
        MonoImage, MonoObject,
    },
    field::ObjectField,
    method::{Arguments, ObjectMethod},
    AsRawVoid, MonoResult, Unbox,
};

pub struct Object {
    pub mono_assembly: *mut MonoAssembly,
    pub mono_class: *mut MonoClass,
    pub mono_domain: *mut MonoDomain,
    pub mono_image: *mut MonoImage,
    pub mono_object: *mut MonoObject,
}

impl Object {
    pub fn construct(&self, args: Option<Arguments>) -> MonoResult<()> {
        unsafe {
            mono_runtime_invoke(
                self.get_method_by_name(".ctor").mono_method,
                self.mono_object as *mut c_void,
                args.as_raw_void() as *mut *mut c_void,
                null_mut(),
            )
        };
        Ok(())
    }

    pub fn get_class_name(&self) -> String {
        unsafe {
            CStr::from_ptr(mono_class_get_name(self.mono_class))
                .to_string_lossy()
                .to_string()
        }
    }

    pub fn get_method_by_name(&self, name: &'static str) -> ObjectMethod {
        let class_name = self.get_class_name();
        let method_name = CString::new(format!("{}:{}()", class_name, name)).unwrap();
        let method_desc = unsafe { mono_method_desc_new(method_name.as_ptr(), 0) };
        let method = unsafe { mono_method_desc_search_in_class(method_desc, self.mono_class) };
        ObjectMethod {
            mono_assembly: self.mono_assembly,
            mono_class: self.mono_class,
            mono_domain: self.mono_domain,
            mono_image: self.mono_image,
            mono_method: method,
            mono_object: self.mono_object,
        }
    }

    pub fn unbox<T>(self) -> T
    where
        T: Unbox,
    {
        T::unbox(self.mono_object)
    }

    pub fn get_field_by_name(&self, name: &'static str) -> MonoResult<ObjectField> {
        let field_name = CString::new(name).unwrap();
        let mono_field =
            unsafe { mono_class_get_field_from_name(self.mono_class, field_name.as_ptr()) };

        if mono_field.is_null() {
            return Err("MonoField Null Error!".into());
        }

        Ok(ObjectField {
            mono_assembly: self.mono_assembly,
            mono_class: self.mono_class,
            mono_domain: self.mono_domain,
            mono_image: self.mono_image,
            mono_field,
            mono_object: self.mono_object,
        })
    }
}

impl AsRawVoid for Object {
    fn as_raw_void(self) -> *mut c_void {
        self.mono_object as *mut c_void
    }
}
