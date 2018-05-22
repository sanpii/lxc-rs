use std::ptr::{null, null_mut};

macro_rules! str {
    ($e:expr) => {{
        let buffer = ::std::ffi::CString::new($e).unwrap();
        let ptr = buffer.as_ptr();

        ::std::mem::forget(buffer);

        ptr
    }};
}

macro_rules! opt_str {
    ($e:expr) => {{
        match $e {
            Some(value) => str!(value),
            None => null(),
        }
    }};
}

pub struct Container {
    inner: *mut ::lxc_sys::lxc_container,
}

impl Container {
    pub fn new(name: &str, config_path: Option<&::std::path::Path>) -> Result<Self, String> {
        let config_path = match config_path {
            Some(path) => str!(path.to_str().unwrap()),
            None => null(),
        };

        let inner = unsafe {
             ::lxc_sys::lxc_container_new(str!(name), config_path)
        };

        Ok(
            Self {
                inner,
            }
        )
    }

    pub fn get(&self) -> Result<(), ()> {
        let success = unsafe {
             ::lxc_sys::lxc_container_get(self.inner)
        };

        if success == 1 {
            Ok(())
        } else {
            Err(())
        }
    }

    pub fn is_defined(&self) -> bool {
        unsafe {
            (*self.inner).is_defined.unwrap()(self.inner)
        }
    }

    pub fn create(&self, template: &str, bdevtype: Option<&str>, specs: Option<&::lxc_sys::bdev_specs>, flags: super::CreateFlags, argv: &[&str]) -> Result<(), ()> {
        let specs = match specs {
            // @TODO
            Some(specs) => null_mut(),
            None => null_mut(),
        };

        let mut argv: Vec<*const i8> = argv.iter()
            .map(|e| str!(*e))
            .collect();

        argv.push(null());

        let result = unsafe {
            (*self.inner).create.unwrap()(
                self.inner,
                str!(template),
                opt_str!(bdevtype),
                specs,
                flags.bits(),
                argv.as_ptr()
            )
        };

        if result {
            Ok(())
        } else {
            Err(())
        }
    }

    pub fn start(&self, use_init: bool, argv: &[&str]) -> Result<(), ()> {
        let argv_ptr = if argv.is_empty() {
            null()
        } else {
            let mut argv: Vec<*const i8> = argv.iter()
                .map(|e| str!(*e))
                .collect();

            argv.push(null());

            argv.as_ptr()
        };

        let success = unsafe {
            (*self.inner).start.unwrap()(self.inner, use_init as i32, argv_ptr)
        };

        if success {
            Ok(())
        } else {
            Err(())
        }
    }

    pub fn state(&self) -> String {
        let state = unsafe {
            ::std::ffi::CStr::from_ptr(
                (*self.inner).state.unwrap()(self.inner)
            )
        };

        state.to_str()
            .unwrap()
            .to_string()
    }

    pub fn init_pid(&self) -> i32 {
        unsafe {
            (*self.inner).init_pid.unwrap()(self.inner)
        }
    }

    pub fn shutdown(&self, timeout: i32) -> Result<(), ()> {
        let success = unsafe {
            (*self.inner).shutdown.unwrap()(self.inner, timeout)
        };

        if success {
            Ok(())
        } else {
            Err(())
        }


    }

    pub fn stop(&self) -> Result<(), ()> {
        let success = unsafe {
            (*self.inner).stop.unwrap()(self.inner)
        };

        if success {
            Ok(())
        } else {
            Err(())
        }
    }

    pub fn destroy(&self) -> Result<(), ()> {
        let success = unsafe {
            (*self.inner).destroy.unwrap()(self.inner)
        };

        if success {
            Ok(())
        } else {
            Err(())
        }
    }
}

impl Drop for Container {
    fn drop(&mut self) {
        unsafe {
            ::lxc_sys::lxc_container_put(self.inner);
        }
    }
}
