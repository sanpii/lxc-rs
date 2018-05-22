extern crate lxc_sys;
#[macro_use]
extern crate bitflags;

mod container;
mod flags;

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

pub use self::container::Container;
pub use self::flags::{AttchFlags, CloneFlags, CreateFlags};

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
