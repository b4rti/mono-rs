[![crates.io](https://img.shields.io/crates/v/mono-rs.svg)](https://crates.io/crates/mono-rs)
[![github/actions](https://github.com/b4rti/mono-rs/actions/workflows/rust.yml/badge.svg?branch=master)](https://github.com/b4rti/mono-rs/actions/workflows/rust.yml)

# mono-rs

## Lightweight Mono wrapper

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

```sh
mcs -target:library -out:Test.dll Test.cs
```

```rust
use mono_rs::{
    bindings::{mono_string_to_utf8, MonoString},
    domain::Domain,
    method::Arguments,
    MonoResult,
};
use std::ffi::CString;

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

    let value_string_object = value_object.mono_ptr as *mut MonoString;
    let value_string = unsafe { mono_string_to_utf8(value_string_object) };
    let value_string = unsafe { CString::from_raw(value_string) };

    println!("Value: {}", &*value_string.to_string_lossy());

    println!("Getting MethodDesc");
    let method = object.get_method_by_name("TestClass:getTestField()")?;

    println!("Calling Method");
    let result_object = method.invoke(Arguments::new())?;
    let result_object = result_object.mono_ptr as *mut MonoString;

    let result_string_object = result_object as *mut MonoString;
    let result_string = unsafe { mono_string_to_utf8(result_string_object) };
    let result_string = unsafe { CString::from_raw(result_string) };

    println!("Result: {}", &*result_string.to_string_lossy());}

    Ok(())
```

```sh
bindgen -o src/bindings.rs src/bindings.h -- -I/usr/include/mono-2.0
```

