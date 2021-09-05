# mono-rs

## Lightweight wrapper aroud mono. !!WIP!! 

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

```bash
mcs -target:library -out:Test.dll Test.cs
```

```rust
use mono_rs::{bindings::*, domain::Domain};
use std::{
    ffi::{c_void, CString},
    ptr::null_mut,
};

fn main() {
	println!("Creating Domain");
	let domain = Domain::new("Default");

	println!("Opening Assembly");
	let assembly = domain.open_assembly("Test.dll");

	println!("Getting Image");
	let image = assembly.get_image();

	println!("Getting Class");
	let class = image.get_class_by_name("TestNS", "TestClass");

	println!("Creating Object");
	let object = class.create_object();

	println!("Calling Constructor");
	object.construct(None);

	////////////////////////////////////////
	// !!WIP!! Raw bindings from here on! //
	////////////////////////////////////////

	println!("Getting Field");
	let class_field_name = CString::new("TestField").unwrap();
	let class_field =
		unsafe { mono_class_get_field_from_name(class.mono_class, class_field_name.as_ptr()) };

	println!("Getting Field Value");
	let value_object = unsafe {
		mono_field_get_value_object(domain.mono_domain, class_field, object.mono_object)
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
}

```
