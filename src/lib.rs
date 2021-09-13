use crate::bindings::{mono_object_unbox, MonoObject};
use std::{
    error::Error,
    ffi::{c_void, CString},
    ptr::null_mut,
};

pub mod array;
pub mod assembly;
pub mod bindings;
pub mod class;
pub mod domain;
pub mod exception;
pub mod field;
pub mod image;
pub mod method;
pub mod object;
pub mod string;
pub mod thread;

type MonoResult<T> = Result<T, Box<dyn Error>>;

pub trait AsRawVoid {
    fn as_raw_void(self) -> *mut c_void;
}

pub trait Unbox {
    fn unbox(object: *mut MonoObject) -> Self;
}

pub trait MonoAutoMarker {}
impl MonoAutoMarker for bool {}
impl MonoAutoMarker for usize {}
impl MonoAutoMarker for isize {}
impl MonoAutoMarker for u8 {}
impl MonoAutoMarker for i8 {}
impl MonoAutoMarker for u16 {}
impl MonoAutoMarker for i16 {}
impl MonoAutoMarker for u32 {}
impl MonoAutoMarker for i32 {}
impl MonoAutoMarker for u64 {}
impl MonoAutoMarker for i64 {}
impl MonoAutoMarker for u128 {}
impl MonoAutoMarker for i128 {}
impl MonoAutoMarker for f32 {}
impl MonoAutoMarker for f64 {}

impl<T> AsRawVoid for T
where
    T: MonoAutoMarker,
{
    fn as_raw_void(self) -> *mut c_void {
        Box::into_raw(Box::new(self)) as *mut _ as *mut c_void
    }
}

impl<T> Unbox for T
where
    T: MonoAutoMarker + Copy,
{
    fn unbox(object: *mut MonoObject) -> Self {
        let val_raw_ptr = unsafe { mono_object_unbox(object) as *mut _ as *mut T };
        unsafe { *val_raw_ptr }
    }
}

impl AsRawVoid for &str {
    fn as_raw_void(self) -> *mut c_void {
        let cstr = Box::new(CString::new(self).unwrap());
        let mut cstr_ptr = cstr.as_ptr();
        Box::into_raw(cstr);
        &mut cstr_ptr as *mut _ as *mut c_void
    }
}

impl AsRawVoid for String {
    fn as_raw_void(self) -> *mut c_void {
        let cstr = Box::new(CString::new(self.as_str()).unwrap());
        let mut cstr_ptr = cstr.as_ptr();
        Box::into_raw(cstr);
        &mut cstr_ptr as *mut _ as *mut c_void
    }
}

impl<T> AsRawVoid for Option<T>
where
    T: AsRawVoid,
{
    fn as_raw_void(self) -> *mut c_void {
        match self {
            Some(t) => t.as_raw_void(),
            None => null_mut() as *mut c_void,
        }
    }
}


#[cfg(test)]
mod tests {
    use crate::{bindings::*, domain::Domain, MonoResult};
    use std::{
        ffi::{c_void, CString},
        ptr::null_mut,
    };

    #[test]
    fn test() -> MonoResult<()> {
        println!("Creating Domain");
        let domain = Domain::new("Default")?;

        println!("Opening Assembly");
        let assembly = domain.open_assembly("Test.dll")?;

        println!("Getting Image");
        let image = assembly.get_image()?;

        println!("Getting Class");
        let class = image.get_class_by_name("TestNS", "TestClass")?;

        println!("Creating Object");
        let object = class.create_object()?;

        println!("Calling Constructor");
        object.construct(None)?;

        println!("Getting Field");
        let field = object.get_field_by_name("TestField")?;

        println!("Getting Field Value");
        let value_object = unsafe {
            mono_field_get_value_object(domain.mono_domain, field.mono_field, object.mono_object)
        };
        let value_string_object = value_object as *mut MonoString;
        let value_string = unsafe { mono_string_to_utf8(value_string_object) };
        let value_string = unsafe { CString::from_raw(value_string) };

        println!("Value: {}", &*value_string.to_string_lossy());

        println!("Getting MethodDesc");
        let method_name = CString::new("TestClass:getTestField()").unwrap();
        let method_decs = unsafe { mono_method_desc_new(method_name.as_ptr(), 0) };
        let method = unsafe { mono_method_desc_search_in_class(method_decs, class.mono_class) };

        println!("Calling Method");
        let result_object = unsafe {
            mono_runtime_invoke(
                method,
                object.mono_object as *mut c_void,
                null_mut(),
                null_mut(),
            )
        };
        let result_string_object = result_object as *mut MonoString;
        let result_string = unsafe { mono_string_to_utf8(result_string_object) };
        let result_string = unsafe { CString::from_raw(result_string) };

        println!("Result: {}", &*result_string.to_string_lossy());

        Ok(())
    }
}
