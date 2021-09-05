use std::ffi::c_void;

use crate::{
    bindings::{MonoAssembly, MonoClass, MonoDomain, MonoImage, MonoMethod, MonoObject},
    void_ptr::MonoVoidPtr,
};

pub struct ClassMethod {
    pub mono_assembly: *mut MonoAssembly,
    pub mono_class: *mut MonoClass,
    pub mono_domain: *mut MonoDomain,
    pub mono_image: *mut MonoImage,
    pub mono_method: *mut MonoMethod,
}

pub struct ObjectMethod {
    pub mono_assembly: *mut MonoAssembly,
    pub mono_class: *mut MonoClass,
    pub mono_domain: *mut MonoDomain,
    pub mono_image: *mut MonoImage,
    pub mono_method: *mut MonoMethod,
    pub mono_object: *mut MonoObject,
}

pub struct StaticMethod {
    pub mono_assembly: *mut MonoAssembly,
    pub mono_class: *mut MonoClass,
    pub mono_domain: *mut MonoDomain,
    pub mono_image: *mut MonoImage,
    pub mono_method: *mut MonoMethod,
}

pub struct Arguments {
    pub args: Vec<*mut c_void>,
}

impl Arguments {
    pub fn new() -> Arguments {
        Arguments { args: Vec::new() }
    }

    pub fn new_with_void_vec(args: Vec<*mut c_void>) -> Arguments {
        Arguments { args }
    }

    pub fn add<T>(&mut self, arg: T)
    where
        T: MonoVoidPtr,
    {
        self.args.push(arg.mono_void_ptr());
    }
}

impl MonoVoidPtr for ClassMethod {
    fn mono_void_ptr(self) -> *mut c_void {
        self.mono_method as *mut c_void
    }
}

impl MonoVoidPtr for ObjectMethod {
    fn mono_void_ptr(self) -> *mut c_void {
        self.mono_method as *mut c_void
    }
}

impl MonoVoidPtr for StaticMethod {
    fn mono_void_ptr(self) -> *mut c_void {
        self.mono_method as *mut c_void
    }
}

impl MonoVoidPtr for Arguments {
    fn mono_void_ptr(mut self) -> *mut c_void {
        self.args.as_mut_ptr() as *mut c_void
    }
}
