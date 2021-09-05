use std::{ffi::c_void, sync::Arc};

use crate::{
    assembly::Assembly,
    bindings::{mono_field_get_value_object, MonoClassField},
    class::Class,
    domain::Domain,
    image::Image,
    object::Object,
    void::AsRawVoid,
    MonoResult,
};

#[derive(Clone, Debug)]
pub struct ClassField {
    pub mono_ptr: *mut MonoClassField,
    pub assembly: Arc<Assembly>,
    pub class: Arc<Class>,
    pub domain: Arc<Domain>,
    pub image: Arc<Image>,
}

#[derive(Clone, Debug)]
pub struct ObjectField {
    pub mono_ptr: *mut MonoClassField,
    pub assembly: Arc<Assembly>,
    pub class: Arc<Class>,
    pub domain: Arc<Domain>,
    pub image: Arc<Image>,
    pub object: Arc<Object>,
}

#[derive(Clone, Debug)]
pub struct StaticField {
    pub mono_ptr: *mut MonoClassField,
    pub assembly: Arc<Assembly>,
    pub class: Arc<Class>,
    pub domain: Arc<Domain>,
    pub image: Arc<Image>,
}

impl ObjectField {
    pub fn get_value_object(&self) -> MonoResult<Object> {
        let mono_ptr = unsafe {
            mono_field_get_value_object(self.domain.mono_ptr, self.mono_ptr, self.object.mono_ptr)
        };

        if mono_ptr.is_null() {
            return Err("MonoField ValueObject Null Error!".into());
        }

        Ok(Object {
            mono_ptr,
            assembly: self.assembly.clone(),
            class: self.class.clone(),
            domain: self.domain.clone(),
            image: self.image.clone(),
        })
    }
}

impl AsRawVoid for ClassField {
    fn as_raw_void(self) -> *mut c_void {
        self.mono_ptr as *mut c_void
    }
}

impl AsRawVoid for ObjectField {
    fn as_raw_void(self) -> *mut c_void {
        self.mono_ptr as *mut c_void
    }
}

impl AsRawVoid for StaticField {
    fn as_raw_void(self) -> *mut c_void {
        self.mono_ptr as *mut c_void
    }
}
