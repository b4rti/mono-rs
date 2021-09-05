use std::ffi::c_void;

use crate::{
    bindings::{MonoAssembly, MonoClass, MonoClassField, MonoDomain, MonoImage, MonoObject},
    void_ptr::MonoVoidPtr,
};

pub struct ClassField {
    pub mono_assembly: *mut MonoAssembly,
    pub mono_class: *mut MonoClass,
    pub mono_domain: *mut MonoDomain,
    pub mono_image: *mut MonoImage,
    pub mono_field: *mut MonoClassField,
}

pub struct ObjectField {
    pub mono_assembly: *mut MonoAssembly,
    pub mono_class: *mut MonoClass,
    pub mono_domain: *mut MonoDomain,
    pub mono_image: *mut MonoImage,
    pub mono_field: *mut MonoClassField,
    pub mono_object: *mut MonoObject,
}

pub struct StaticField {
    pub mono_assembly: *mut MonoAssembly,
    pub mono_class: *mut MonoClass,
    pub mono_domain: *mut MonoDomain,
    pub mono_image: *mut MonoImage,
    pub mono_field: *mut MonoClassField,
}

impl MonoVoidPtr for ClassField {
    fn as_void_ptr(self) -> *mut c_void {
        self.mono_field as *mut c_void
    }
}

impl MonoVoidPtr for ObjectField {
    fn as_void_ptr(self) -> *mut c_void {
        self.mono_field as *mut c_void
    }
}

impl MonoVoidPtr for StaticField {
    fn as_void_ptr(self) -> *mut c_void {
        self.mono_field as *mut c_void
    }
}
