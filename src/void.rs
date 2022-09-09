use std::{
    ffi::{c_void, CString},
    ptr::null_mut,
};

pub trait AsRawVoidMarker {}
impl AsRawVoidMarker for bool {}
impl AsRawVoidMarker for usize {}
impl AsRawVoidMarker for isize {}
impl AsRawVoidMarker for u8 {}
impl AsRawVoidMarker for i8 {}
impl AsRawVoidMarker for u16 {}
impl AsRawVoidMarker for i16 {}
impl AsRawVoidMarker for u32 {}
impl AsRawVoidMarker for i32 {}
impl AsRawVoidMarker for u64 {}
impl AsRawVoidMarker for i64 {}
impl AsRawVoidMarker for f32 {}
impl AsRawVoidMarker for f64 {}

pub trait AsRawVoid {
    fn as_raw_void(self) -> *mut c_void;
}

impl<T> AsRawVoid for T
where
    T: AsRawVoidMarker,
{
    fn as_raw_void(self) -> *mut c_void {
        Box::into_raw(Box::new(self)) as *mut _ as *mut c_void
    }
}

impl AsRawVoid for &str {
    fn as_raw_void(self) -> *mut c_void {
        let cstr = Box::new(CString::new(self).unwrap());
        let mut cstr_ptr = cstr.as_ptr();
        Box::leak(cstr);
        &mut cstr_ptr as *mut _ as *mut c_void
    }
}

impl AsRawVoid for String {
    fn as_raw_void(self) -> *mut c_void {
        let cstr = Box::new(CString::new(self.as_str()).unwrap());
        let mut cstr_ptr = cstr.as_ptr();
        Box::leak(cstr);
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
