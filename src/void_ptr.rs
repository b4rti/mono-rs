use std::{
    ffi::{c_void, CString},
    ptr::null_mut,
};

pub trait MonoVoidPtr {
    fn mono_void_ptr(self) -> *mut c_void;
}

pub trait MonoVoidPtrAuto {}
impl MonoVoidPtrAuto for bool {}
impl MonoVoidPtrAuto for usize {}
impl MonoVoidPtrAuto for isize {}
impl MonoVoidPtrAuto for u8 {}
impl MonoVoidPtrAuto for i8 {}
impl MonoVoidPtrAuto for u16 {}
impl MonoVoidPtrAuto for i16 {}
impl MonoVoidPtrAuto for u32 {}
impl MonoVoidPtrAuto for i32 {}
impl MonoVoidPtrAuto for u64 {}
impl MonoVoidPtrAuto for i64 {}
impl MonoVoidPtrAuto for u128 {}
impl MonoVoidPtrAuto for i128 {}
impl MonoVoidPtrAuto for f32 {}
impl MonoVoidPtrAuto for f64 {}

impl<T> MonoVoidPtr for T
where
    T: MonoVoidPtrAuto,
{
    fn mono_void_ptr(self) -> *mut c_void {
        Box::into_raw(Box::new(self)) as *mut _ as *mut c_void
    }
}

impl MonoVoidPtr for &str {
    fn mono_void_ptr(self) -> *mut c_void {
        let cstr = Box::new(CString::new(self).unwrap());
        let mut cstr_ptr = cstr.as_ptr();
        Box::into_raw(cstr);
        &mut cstr_ptr as *mut _ as *mut c_void
    }
}

impl MonoVoidPtr for String {
    fn mono_void_ptr(self) -> *mut c_void {
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
    fn mono_void_ptr(self) -> *mut c_void {
        match self {
            Some(t) => t.mono_void_ptr(),
            None => null_mut() as *mut c_void,
        }
    }
}
