use std::ptr::{null, null_mut};

macro_rules! opt_str {
    ($e:expr) => {
        match $e {
            Some(value) => string!(value),
            None => null(),
        }
    };
}

macro_rules! call {
    ( $container:ident . $method:ident ) => {
        call!($container.$method, )
    };

    ( $container:ident . $method:ident, $( $arg:expr ),* ) => {
        unsafe {
            (*$container.inner).$method.unwrap()($container.inner, $($arg,)*)
        }
    };
}

pub struct Container {
    inner: *mut ::lxc_sys::lxc_container,
}

impl Container {
    pub fn new(name: &str, config_path: Option<&::std::path::Path>) -> Result<Self, String> {
        let config_path = match config_path {
            Some(path) => string!(path.to_str().unwrap()),
            None => null(),
        };

        let inner = unsafe {
             ::lxc_sys::lxc_container_new(string!(name), config_path)
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

        if success == 0 {
            Ok(())
        } else {
            Err(())
        }
    }

    pub fn is_defined(&self) -> bool {
        call!(self.is_defined)
    }

    pub fn create(
        &self,
        template: &str,
        bdevtype: Option<&str>,
        specs: Option<&::lxc_sys::bdev_specs>,
        flags: super::CreateFlags,
        argv: &[&str],
    ) -> Result<(), ()> {
        let specs = match specs {
            // @TODO
            Some(specs) => null_mut(),
            None => null_mut(),
        };

        let mut argv: Vec<*const i8> = argv.iter()
            .map(|e| string!(*e))
            .collect();

        argv.push(null());

        let result = call!(
            self.create,
            string!(template),
            opt_str!(bdevtype),
            specs,
            flags.bits(),
            argv.as_ptr()
        );

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
                .map(|e| string!(*e))
                .collect();

            argv.push(null());

            argv.as_ptr()
        };

        let success = call!(self.start, use_init as i32, argv_ptr);

        if success {
            Ok(())
        } else {
            Err(())
        }
    }

    pub fn state(&self) -> String {
        #[allow(unused_unsafe)]
        let state = unsafe {
            ::std::ffi::CStr::from_ptr(
                call!(self.state)
            )
        };

        state.to_str()
            .unwrap()
            .to_string()
    }

    pub fn init_pid(&self) -> i32 {
        call!(self.init_pid)
    }

    pub fn shutdown(&self, timeout: i32) -> Result<(), ()> {
        let success = call!(self.shutdown, timeout);

        if success {
            Ok(())
        } else {
            Err(())
        }
    }

    pub fn stop(&self) -> Result<(), ()> {
        let success = call!(self.stop);

        if success {
            Ok(())
        } else {
            Err(())
        }
    }

    pub fn destroy(&self) -> Result<(), ()> {
        let success = call!(self.destroy);

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
