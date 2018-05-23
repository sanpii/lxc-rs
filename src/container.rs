use std::ptr::{null, null_mut};

macro_rules! call {
    ( $container:ident . $method:ident( $( $arg:expr ),* ) ) => {
        unsafe {
            (*$container.inner).$method.unwrap()($container.inner, $($arg,)*)
        }
    };

    ( $container:ident . $method:ident( $( $arg:expr ),* ) -> c_str ) => {{
        let result = unsafe {
            (*$container.inner).$method.unwrap()($container.inner, $($arg,)*)
        };

        let str = unsafe {
            ::std::ffi::CStr::from_ptr(result)
        };

        str.to_str()
            .unwrap()
            .to_string()
    }};

    ( $container:ident . $method:ident( $( $arg:expr ),* ) -> bool ) => {{
        let result = unsafe {
            (*$container.inner).$method.unwrap()($container.inner, $($arg,)*)
        };

        if result {
            Ok(())
        } else {
            Err(())
        }
    }};
}

pub struct Container {
    inner: *mut ::lxc_sys::lxc_container,
}

impl Container {
    pub fn new(name: &str, config_path: Option<&::std::path::Path>) -> Result<Self, String> {
        let config_path = match config_path {
            Some(path) => super::ffi::to_cstr(path.to_str().unwrap()),
            None => null(),
        };

        let inner = unsafe {
             ::lxc_sys::lxc_container_new(super::ffi::to_cstr(name), config_path)
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
        call!(self.is_defined())
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
            .map(|e| super::ffi::to_cstr(*e))
            .collect();

        argv.push(null());

        call!(
            self.create(
                super::ffi::to_cstr(template),
                match bdevtype {
                    Some(value) => super::ffi::to_cstr(value),
                    None => null(),
                },
                specs,
                flags.bits(),
                argv.as_ptr()
            ) -> bool
        )
    }

    pub fn start(&self, use_init: bool, argv: &[&str]) -> Result<(), ()> {
        let argv_ptr = if argv.is_empty() {
            null()
        } else {
            let mut argv: Vec<*const i8> = argv.iter()
                .map(|e| super::ffi::to_cstr(*e))
                .collect();

            argv.push(null());

            argv.as_ptr()
        };

        call!(self.start(use_init as i32, argv_ptr) -> bool)
    }

    pub fn state(&self) -> String {
        call!(self.state() -> c_str)
    }

    pub fn init_pid(&self) -> i32 {
        call!(self.init_pid())
    }

    pub fn shutdown(&self, timeout: i32) -> Result<(), ()> {
        call!(self.shutdown(timeout) -> bool)
    }

    pub fn stop(&self) -> Result<(), ()> {
        call!(self.stop() -> bool)
    }

    pub fn destroy(&self) -> Result<(), ()> {
        call!(self.destroy() -> bool)
    }
}

impl Drop for Container {
    fn drop(&mut self) {
        unsafe {
            ::lxc_sys::lxc_container_put(self.inner);
        }
    }
}
