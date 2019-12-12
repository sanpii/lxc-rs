pub mod attach;
mod console;
mod container;
mod ffi;
mod flags;
pub mod log;
mod migrate;

pub use self::container::Container;
pub use self::flags::{AttchFlags, CloneFlags, CreateFlags};
pub use self::log::Log;

pub use lxc_sys::lxc_conf as Conf;
pub use lxc_sys::lxc_lock as Lock;
pub use lxc_sys::lxc_snapshot as Snapshot;

#[derive(Debug)]
pub struct Error {
    pub num: i32,
    pub str: String,
}

impl std::fmt::Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{}", self.str)
    }
}

impl std::error::Error for Error {}

pub type Result<T> = std::result::Result<T, Error>;

/**
 * Determine version of LXC.
 */
pub fn version() -> String {
    let version = unsafe { std::ffi::CStr::from_ptr(lxc_sys::lxc_get_version()) };

    version.to_str().unwrap().to_string()
}

/**
 * Obtain a list of all container states.
 */
pub fn wait_states() -> Vec<String> {
    let size = unsafe { lxc_sys::lxc_get_wait_states(std::ptr::null_mut()) };

    let mut states = Vec::new();
    states.resize(size as usize, std::ptr::null());

    unsafe { lxc_sys::lxc_get_wait_states(states.as_mut_ptr()) };

    states.iter().map(|e| self::ffi::to_string(*e)).collect()
}

/**
 * Get the value for a global config key.
 */
pub fn get_global_config_item(key: &str) -> Option<String> {
    let c_key = self::ffi::to_cstr(key);
    let value = unsafe { lxc_sys::lxc_get_global_config_item(c_key) };
    self::ffi::release(c_key);

    if value.is_null() {
        None
    } else {
        Some(self::ffi::to_string(value))
    }
}

/**
 * Check if the configuration item is supported by this LXC instance.
 */
#[cfg(feature = "v2_1")]
pub fn config_item_is_supported(key: &str) -> bool {
    let c_key = self::ffi::to_cstr(key);
    let r = unsafe { lxc_sys::lxc_config_item_is_supported(c_key) };
    self::ffi::release(c_key);
    r
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
