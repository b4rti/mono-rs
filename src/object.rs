use std::{
    ffi::{c_void, CStr, CString},
    ptr::null_mut,
    sync::Arc,
};

use crate::{
    assembly::Assembly,
    bindings::{
        mono_class_get_field_from_name, mono_class_get_name, mono_method_desc_new,
        mono_method_desc_search_in_class, mono_runtime_invoke, MonoObject,
    },
    class::Class,
    domain::Domain,
    field::ObjectField,
    image::Image,
    method::{Arguments, ObjectMethod},
    void::AsRawVoid,
    MonoResult,
};

#[derive(Clone, Debug)]
pub struct Object {
    pub mono_ptr: *mut MonoObject,
    pub assembly: Arc<Assembly>,
    pub class: Arc<Class>,
    pub domain: Arc<Domain>,
    pub image: Arc<Image>,
}

impl Object {
    pub fn construct(&self, args: Option<Arguments>) -> MonoResult<()> {
        unsafe {
            mono_runtime_invoke(
                self.get_method_by_name(".ctor")?.mono_ptr,
                self.mono_ptr as *mut c_void,
                args.as_raw_void() as *mut *mut c_void,
                null_mut(),
            )
        };
        Ok(())
    }

    pub fn get_class_name(&self) -> String {
        unsafe {
            CStr::from_ptr(mono_class_get_name(self.class.mono_ptr))
                .to_string_lossy()
                .to_string()
        }
    }

    pub fn get_method_by_name<T: AsRef<str>>(&self, name: T) -> MonoResult<ObjectMethod> {
        let class_name = self.get_class_name();
        let method_name = CString::new(format!("{}:{}()", class_name, name.as_ref()))?;
        let method_desc = unsafe { mono_method_desc_new(method_name.as_ptr(), 0) };
        let mono_ptr = unsafe { mono_method_desc_search_in_class(method_desc, self.class.mono_ptr) };

        if mono_ptr.is_null() {
            return Err("MonoMethod Null Error!".into());
        }

        Ok(ObjectMethod {
            mono_ptr,
            assembly: self.assembly.clone(),
            class: self.class.clone(),
            domain: self.domain.clone(),
            image: self.image.clone(),
            object: Arc::new(self.clone()),
        })
    }

    pub fn get_field_by_name<T: AsRef<str>>(&self, name: T) -> MonoResult<ObjectField> {
        let field_name = CString::new(name.as_ref())?;
        let mono_ptr =
            unsafe { mono_class_get_field_from_name(self.class.mono_ptr, field_name.as_ptr()) };

        if mono_ptr.is_null() {
            return Err("MonoField Null Error!".into());
        }

        Ok(ObjectField {
            mono_ptr,
            assembly: self.assembly.clone(),
            class: self.class.clone(),
            domain: self.domain.clone(),
            image: self.image.clone(),
            object: Arc::new(self.clone()),
        })
    }
}

impl AsRawVoid for Object {
    fn as_raw_void(self) -> *mut c_void {
        self.mono_ptr as *mut c_void
    }
}
