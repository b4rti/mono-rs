use std::{
    ffi::{c_void, CString},
    ptr::null_mut,
};

use crate::bindings::{mono_object_unbox, MonoObject};

pub trait MonoVoidPtr {
    fn to_mono_ptr(self) -> *mut c_void;
}

pub trait MonoUnbox {
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

impl<T> MonoVoidPtr for T
where
    T: MonoAutoMarker,
{
    fn to_mono_ptr(self) -> *mut c_void {
        Box::into_raw(Box::new(self)) as *mut _ as *mut c_void
    }
}

impl<T> MonoUnbox for T
where
    T: MonoAutoMarker + Copy,
{
    fn unbox(object: *mut MonoObject) -> Self {
        let val_raw_ptr = unsafe { mono_object_unbox(object) as *mut _ as *mut T };
        unsafe { *val_raw_ptr }
    }
}

impl MonoVoidPtr for &str {
    fn to_mono_ptr(self) -> *mut c_void {
        let cstr = Box::new(CString::new(self).unwrap());
        let mut cstr_ptr = cstr.as_ptr();
        Box::into_raw(cstr);
        &mut cstr_ptr as *mut _ as *mut c_void
    }
}

impl MonoVoidPtr for String {
    fn to_mono_ptr(self) -> *mut c_void {
        let cstr = Box::new(CString::new(self.as_str()).unwrap());
        let mut cstr_ptr = cstr.as_ptr();
        Box::into_raw(cstr);
        &mut cstr_ptr as *mut _ as *mut c_void
    }
}

impl<T> MonoVoidPtr for Option<T>
where
    T: MonoVoidPtr,
{
    fn to_mono_ptr(self) -> *mut c_void {
        match self {
            Some(t) => t.to_mono_ptr(),
            None => null_mut() as *mut c_void,
        }
    }
}
