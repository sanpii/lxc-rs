#![warn(warnings)]

#[macro_use]
mod ffi;

pub mod attach;
mod console;
mod container;
pub mod containers;
mod flags;
pub mod log;
mod migrate;

pub use container::Container;
pub use flags::{AttchFlags, CloneFlags, CreateFlags};
pub use log::Log;

pub use lxc_sys::lxc_conf as Conf;
pub use lxc_sys::lxc_lock as Lock;
#[cfg(feature = "v3_1")]
pub use lxc_sys::lxc_mount as Mount;
pub use lxc_sys::lxc_snapshot as Snapshot;

#[derive(Debug)]
pub struct Error {
    pub num: i32,
    pub str: String,
}

impl std::fmt::Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.str)
    }
}

impl std::error::Error for Error {}

pub type Result<T = ()> = std::result::Result<T, Error>;

/**
 * Determine version of LXC.
 */
#[must_use]
pub fn version() -> String {
    let version = unsafe { std::ffi::CStr::from_ptr(lxc_sys::lxc_get_version()) };

    version.to_str().unwrap().to_string()
}

/**
 * Obtain a list of all container states.
 */
#[must_use]
pub fn wait_states() -> Vec<String> {
    let size = unsafe { lxc_sys::lxc_get_wait_states(std::ptr::null_mut()) };

    let mut states = Vec::new();
    states.resize(size as usize, std::ptr::null());

    unsafe { lxc_sys::lxc_get_wait_states(states.as_mut_ptr()) };

    states.iter().map(|e| ffi::to_string(*e)).collect()
}

/**
 * Get the value for a global config key.
 */
#[must_use]
pub fn get_global_config_item(key: &str) -> Option<String> {
    let value = unsafe { lxc_sys::lxc_get_global_config_item(cstr!(key)) };

    if value.is_null() {
        None
    } else {
        Some(ffi::to_string(value))
    }
}

/**
 * Check if the configuration item is supported by this LXC instance.
 */
#[cfg(feature = "v2_1")]
pub fn config_item_is_supported(key: &str) -> bool {
    unsafe { lxc_sys::lxc_config_item_is_supported(cstr!(key)) }
}

/**
 * Retrieves the default path where LXC containers are stored.
 *
 * This function retrieves the value of the global configuration item "lxc.lxcpath".
 * The returned path is used as the root directory for managing LXC containers.
 *
 * # Return
 *
 * * `Option<String>` - Returns an `Option` containing a string representing the LXC path.
 *   - `Some(String)` - If the global configuration item is found and its value is not empty,
 *     the function returns `Some` containing the LXC path.
 *   - `None`         - If the global configuration item is not found or its value is empty,
 *     the function returns `None`.
 */
pub fn path() -> Option<String> {
    get_global_config_item("lxc.lxcpath")
}
