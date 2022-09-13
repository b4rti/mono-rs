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
    use crate::{domain::Domain, value::Value, MonoResult};

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
        if let Value::Str(value_string) = value_object.try_into()? {
            println!("FieldValue: {}", value_string);
        } else {
            println!("FieldValue is not a string");
        }

        println!("Getting MethodDesc");
        let method = object.get_method_by_name("TestClass:getTestField()")?;

        println!("Calling Method");
        let result_object = method.invoke(None)?;

        if let Value::Str(result_string) = result_object.try_into()? {
            println!("{}", result_string);
        } else {
            println!("Result is not a string");
        }

        Ok(())
    }
}
