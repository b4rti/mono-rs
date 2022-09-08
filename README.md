![crates.io](https://img.shields.io/crates/v/mono-rs.svg)

# mono-rs

## Lightweight wrapper around mono. !!WIP!! 

### Example

```csharp
using System;

namespace TestNS
{
    class TestClass
    {
        String TestField = "Test";

        String getTestField()
        {
            return "Call result: " + this.TestField;
        }
    }
}
```

```shell
mcs -target:library -out:Test.dll Test.cs
```

```rust
use mono_rs::{bindings::*, domain::Domain, MonoResult};
use std::{
    ffi::{c_void, CString},
    ptr::null_mut,
};

fn main() -> MonoResult<()> {
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
    
    
    ////////////////////////////////////////
    // !!WIP!! Raw bindings from here on! //
    ////////////////////////////////////////
    
    
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

```
