use std::{
    ffi::{c_void, CString},
    path::Path,
    sync::Arc,
};

use crate::{
    assembly::Assembly,
    bindings::{mono_domain_assembly_open, mono_jit_init, mono_jit_init_version, MonoDomain},
    void::AsRawVoid,
    MonoResult,
};

#[derive(Clone, Debug)]
pub struct Domain {
    pub mono_ptr: *mut MonoDomain,
}

impl Domain {
    pub fn new<T: AsRef<str>>(name: T) -> MonoResult<Self> {
        let name = CString::new(name.as_ref())?;
        Ok(Domain {
            mono_ptr: unsafe { mono_jit_init(name.as_ptr()) },
        })
    }

    pub fn new_with_version<T: AsRef<str>>(name: T, version: T) -> MonoResult<Self> {
        let name = CString::new(name.as_ref())?;
        let version = CString::new(version.as_ref())?;
        let mono_ptr = unsafe { mono_jit_init_version(name.as_ptr(), version.as_ptr()) };

        if mono_ptr.is_null() {
            return Err("MonoDomain Null Error!".into());
        }

        Ok(Domain { mono_ptr })
    }

    pub fn open_assembly<T: AsRef<str>>(&self, path: T) -> MonoResult<Assembly> {
        if !Path::new(path.as_ref()).exists() {
            return Err(format!("File '{}' not found!", path.as_ref()).into());
        }

        let path = CString::new(path.as_ref())?;
        let mono_ptr =
            unsafe { mono_domain_assembly_open(self.mono_ptr, path.as_ptr()) };

        if mono_ptr.is_null() {
            return Err("MonoAssembly Null Error!".into());
        }

        Ok(Assembly {
            mono_ptr,
            domain: Arc::new(self.clone()),
        })
    }
}

impl AsRawVoid for Domain {
    fn as_raw_void(self) -> *mut c_void {
        self.mono_ptr as *mut c_void
    }
}
