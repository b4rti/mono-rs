use std::ffi::{c_void, CString};

use crate::{
    assembly::Assembly,
    bindings::{mono_domain_assembly_open, mono_jit_init, mono_jit_init_version, MonoDomain},
    void_ptr::MonoVoidPtr,
};

pub struct Domain {
    pub mono_domain: *mut MonoDomain,
}

impl Domain {
    pub fn new(name: &'static str) -> Domain {
        let domain_name = CString::new(name).unwrap();
        Domain {
            mono_domain: unsafe { mono_jit_init(domain_name.as_ptr()) },
        }
    }

    pub fn new_with_version(name: &'static str, version: &'static str) -> Domain {
        let domain_name = CString::new(name).unwrap();
        let domain_version = CString::new(version).unwrap();
        Domain {
            mono_domain: unsafe {
                mono_jit_init_version(domain_name.as_ptr(), domain_version.as_ptr())
            },
        }
    }

    pub fn open_assembly(&self, path: &'static str) -> Assembly {
        let assembly_path = CString::new(path).unwrap();
        Assembly {
            mono_domain: self.mono_domain,
            mono_assembly: unsafe {
                mono_domain_assembly_open(self.mono_domain, assembly_path.as_ptr())
            },
        }
    }
}

impl MonoVoidPtr for Domain {
    fn mono_void_ptr(self) -> *mut c_void {
        self.mono_domain as *mut c_void
    }
}
