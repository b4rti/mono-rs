use std::{ffi::{c_void, CString}, path::Path};

use crate::{
    assembly::Assembly,
    bindings::{mono_domain_assembly_open, mono_jit_init, mono_jit_init_version, MonoDomain},
    AsRawVoid, MonoResult,
};

pub struct Domain {
    pub mono_domain: *mut MonoDomain,
}

impl Domain {
    pub fn new<T: AsRef<str>>(name: T) -> MonoResult<Self> {
        let domain_name = CString::new(name.as_ref())?;
        Ok(Domain {
            mono_domain: unsafe { mono_jit_init(domain_name.as_ptr()) },
        })
    }

    pub fn new_with_version<T: AsRef<str>>(name: T, version: T) -> MonoResult<Self> {
        let domain_name = CString::new(name.as_ref())?;
        let domain_version = CString::new(version.as_ref())?;
        let mono_domain =
            unsafe { mono_jit_init_version(domain_name.as_ptr(), domain_version.as_ptr()) };

        if mono_domain.is_null() {
            return Err("MonoDomain Null Error!".into());
        }

        Ok(Domain { mono_domain })
    }

    pub fn open_assembly<T: AsRef<str>>(&self, path: T) -> MonoResult<Assembly> {
        if !Path::new(path.as_ref()).exists() {
            return Err(format!("File '{}' not found!", path.as_ref()).into());
        }

        let assembly_path = CString::new(path.as_ref())?;
        let mono_assembly = unsafe {
                mono_domain_assembly_open(self.mono_domain, assembly_path.as_ptr())
        };

        if mono_assembly.is_null() {
            return Err("MonoAssembly Null Error!".into());
        }

        Ok(Assembly {
            mono_domain: self.mono_domain,
            mono_assembly,
        })
    }
}

impl AsRawVoid for Domain {
    fn as_raw_void(self) -> *mut c_void {
        self.mono_domain as *mut c_void
    }
}
