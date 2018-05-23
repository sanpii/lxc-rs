extern crate lxc_sys;
#[macro_use]
extern crate bitflags;

macro_rules! string {
    ($e:expr) => {{
        let buffer = ::std::ffi::CString::new($e).unwrap();
        let ptr = buffer.as_ptr();

        ::std::mem::forget(buffer);

        ptr
    }};
}

macro_rules! str {
    ($e:expr) => {{
        let buffer = unsafe {
            ::std::ffi::CStr::from_ptr($e)
        };

        buffer.to_str()
            .unwrap()
            .to_string()
    }};
}

mod container;
mod flags;
pub mod log;

pub use self::container::Container;
pub use self::flags::{AttchFlags, CloneFlags, CreateFlags};
pub use self::log::Log;

pub fn version() -> String {
    let version = unsafe {
        ::std::ffi::CStr::from_ptr(::lxc_sys::lxc_get_version())
    };

    version.to_str()
        .unwrap()
        .to_string()
}

pub fn wait_states() -> Vec<String> {
    let size = unsafe {
        ::lxc_sys::lxc_get_wait_states(::std::ptr::null_mut())
    };

    let mut states = Vec::new();
    states.resize(size as usize, ::std::ptr::null());

    unsafe {
        ::lxc_sys::lxc_get_wait_states(states.as_mut_ptr())
    };

    states.iter()
        .map(|e| str!(*e))
        .collect()
}

pub fn get_global_config_item(key: &str) -> Result<String, ()> {
    let value = unsafe {
        ::lxc_sys::lxc_get_global_config_item(string!(key))
    };

    if value == ::std::ptr::null() {
        Err(())
    } else {
        Ok(str!(value))
    }
}

#[cfg(feature = "v2_0")]
pub fn config_item_is_supported(key: &str) -> bool {
    unsafe {
        ::lxc_sys::lxc_config_item_is_supported(string!(key))
    }
}
