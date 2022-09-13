use std::{
    collections::HashMap,
    convert::TryFrom,
    error::Error,
    ffi::{c_void, CString},
    ptr::null_mut,
};

use crate::{
    bindings::{mono_string_to_utf8, MonoString},
    object::Object,
    void::AsRawVoid,
};

#[derive(Clone, Debug)]
pub enum Value {
    Void(()),
    Bool(bool),
    UInt8(u8),
    Int8(i8),
    Char(char),
    UInt16(u16),
    Int16(i16),
    UInt32(u32),
    Int32(i32),
    UInt64(u64),
    Int64(i64),
    Size(isize),
    USize(usize),
    Float(f32),
    Double(f64),
    Str(String),
    Object(Object),
    Array(Vec<Value>),
    Dict(HashMap<Value, Value>),
    Enum(String, Box<Value>),
    Ptr(isize),
}

impl TryFrom<Object> for Value {
    type Error = Box<dyn Error>;

    fn try_from(object: Object) -> Result<Self, Self::Error> {
        // TODO: Implement TryFrom for all types
        let value_string_object = object.mono_ptr as *mut MonoString;
        let value_string = unsafe { mono_string_to_utf8(value_string_object) };
        let value_string = unsafe { CString::from_raw(value_string) };

        Ok(Value::Str(String::from(value_string.to_str()?)))
    }
}

impl AsRawVoid for Value {
    fn as_raw_void(self) -> *mut c_void {
        match self {
            Value::Void(_) => null_mut() as *mut c_void,
            Value::Bool(value) => value.as_raw_void(),
            Value::UInt8(value) => value.as_raw_void(),
            Value::Int8(value) => value.as_raw_void(),
            Value::Char(value) => value.as_raw_void(),
            Value::UInt16(value) => value.as_raw_void(),
            Value::Int16(value) => value.as_raw_void(),
            Value::UInt32(value) => value.as_raw_void(),
            Value::Int32(value) => value.as_raw_void(),
            Value::UInt64(value) => value.as_raw_void(),
            Value::Int64(value) => value.as_raw_void(),
            Value::Size(value) => value.as_raw_void(),
            Value::USize(value) => value.as_raw_void(),
            Value::Float(value) => value.as_raw_void(),
            Value::Double(value) => value.as_raw_void(),
            Value::Str(value) => value.as_raw_void(),
            Value::Object(value) => value.as_raw_void(),
            Value::Array(_) => null_mut() as *mut c_void,
            Value::Dict(_) => null_mut() as *mut c_void,
            Value::Enum(_key, value) => value.as_raw_void(),
            Value::Ptr(value) => value.as_raw_void(),
        }
    }
}
