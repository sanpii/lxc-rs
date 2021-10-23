#![warn(rust_2018_idioms)]

#[macro_use]
mod ffi;

pub mod attach;
mod console;
mod container;
mod errors;
mod flags;
pub mod log;
mod migrate;

pub use self::container::Container;
pub use self::errors::{Error, Result};
pub use self::flags::{AttchFlags, CloneFlags, CreateFlags};
pub use self::log::Log;

pub use lxc_sys::lxc_conf as Conf;
pub use lxc_sys::lxc_lock as Lock;
#[cfg(feature = "v3_1")]
pub use lxc_sys::lxc_mount as Mount;
pub use lxc_sys::lxc_snapshot as Snapshot;

/**
 * Determine version of LXC.
 */
pub fn version() -> crate::Result<String> {
    let version = unsafe { std::ffi::CStr::from_ptr(lxc_sys::lxc_get_version()) };

    Ok(version.to_str()?.to_string())
}

/**
 * Obtain a list of all container states.
 */
pub fn wait_states() -> crate::Result<Vec<String>> {
    let size = unsafe { lxc_sys::lxc_get_wait_states(std::ptr::null_mut()) };

    let mut states = Vec::new();
    states.resize(size as usize, std::ptr::null());

    unsafe { lxc_sys::lxc_get_wait_states(states.as_mut_ptr()) };

    states.iter().map(|e| self::ffi::to_string(*e)).collect()
}

/**
 * Get the value for a global config key.
 */
pub fn get_global_config_item(key: &str) -> crate::Result<Option<String>> {
    let value = unsafe { lxc_sys::lxc_get_global_config_item(cstr!(key)) };

    if value.is_null() {
        Ok(None)
    } else {
        Some(self::ffi::to_string(value)).transpose()
    }
}

/**
 * Check if the configuration item is supported by this LXC instance.
 */
#[cfg(feature = "v2_1")]
pub fn config_item_is_supported(key: &str) -> crate::Result<bool> {
    let is_supported = unsafe { lxc_sys::lxc_config_item_is_supported(cstr!(key)) };

    Ok(is_supported)
}

pub fn list_active_containers() {
    unimplemented!();
}

pub fn list_all_containers() {
    unimplemented!();
}

pub fn list_defined_containers() {
    unimplemented!();
}
