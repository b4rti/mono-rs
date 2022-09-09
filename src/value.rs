use std::{collections::HashMap, convert::TryFrom, error::Error};

use crate::object::Object;

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
        let _object = object;
        Ok(Value::Bool(false))
    }
}
