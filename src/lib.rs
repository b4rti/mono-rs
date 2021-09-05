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
pub mod value;
pub mod void;

use std::error::Error;
pub type MonoResult<T> = Result<T, Box<dyn Error>>;

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
        let value_object = field.get_value_object()?;

        let value_string_object = value_object.mono_ptr as *mut MonoString;
        let value_string = unsafe { mono_string_to_utf8(value_string_object) };
        let value_string = unsafe { CString::from_raw(value_string) };

        println!("Value: {}", &*value_string.to_string_lossy());

        println!("Getting MethodDesc");
        let method_name = CString::new("TestClass:getTestField()")?;
        let method_decs = unsafe { mono_method_desc_new(method_name.as_ptr(), 0) };
        let method = unsafe { mono_method_desc_search_in_class(method_decs, class.mono_ptr) };

        println!("Calling Method");
        let result_object = unsafe {
            mono_runtime_invoke(
                method,
                object.mono_ptr as *mut c_void,
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
