use std::ffi::c_void;

use crate::{
    bindings::{MonoAssembly, MonoClass, MonoClassField, MonoDomain, MonoImage, MonoObject},
    AsRawVoid,
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

impl AsRawVoid for ClassField {
    fn as_raw_void(self) -> *mut c_void {
        self.mono_field as *mut c_void
    }
}

impl AsRawVoid for ObjectField {
    fn as_raw_void(self) -> *mut c_void {
        self.mono_field as *mut c_void
    }
}

impl AsRawVoid for StaticField {
    fn as_raw_void(self) -> *mut c_void {
        self.mono_field as *mut c_void
    }
}
