use std::{ffi::c_void, sync::Arc};

use crate::{
    assembly::Assembly,
    bindings::{mono_runtime_invoke, MonoMethod, MonoObject},
    class::Class,
    domain::Domain,
    image::Image,
    object::Object,
    value::Value,
    void::AsRawVoid,
    MonoResult,
};

#[derive(Clone, Debug)]
pub struct ClassMethod {
    pub mono_ptr: *mut MonoMethod,
    pub assembly: Arc<Assembly>,
    pub class: Arc<Class>,
    pub domain: Arc<Domain>,
    pub image: Arc<Image>,
}

#[derive(Clone, Debug)]
pub struct ObjectMethod {
    pub mono_ptr: *mut MonoMethod,
    pub assembly: Arc<Assembly>,
    pub class: Arc<Class>,
    pub domain: Arc<Domain>,
    pub image: Arc<Image>,
    pub object: Arc<Object>,
}

impl ObjectMethod {
    pub fn invoke(&self, args: Option<&[Value]>) -> MonoResult<Object> {
        let mono_result = unsafe {
            mono_runtime_invoke(
                self.mono_ptr,
                self.object.mono_ptr as *mut c_void,
                args.as_raw_void() as *mut *mut c_void,
                std::ptr::null_mut(),
            )
        };
        if mono_result.is_null() {
            return Err("Method returned null".into());
        }

        let result = Object {
            mono_ptr: mono_result as *mut MonoObject,
            assembly: self.assembly.clone(),
            class: self.class.clone(),
            domain: self.domain.clone(),
            image: self.image.clone(),
        };

        Ok(result)
    }
}

#[derive(Clone, Debug)]
pub struct StaticMethod {
    pub mono_ptr: *mut MonoMethod,
    pub assembly: Arc<Assembly>,
    pub class: Arc<Class>,
    pub domain: Arc<Domain>,
    pub image: Arc<Image>,
}

impl AsRawVoid for ClassMethod {
    fn as_raw_void(self) -> *mut c_void {
        self.mono_ptr as *mut c_void
    }
}

impl AsRawVoid for ObjectMethod {
    fn as_raw_void(self) -> *mut c_void {
        self.mono_ptr as *mut c_void
    }
}

impl AsRawVoid for StaticMethod {
    fn as_raw_void(self) -> *mut c_void {
        self.mono_ptr as *mut c_void
    }
}
