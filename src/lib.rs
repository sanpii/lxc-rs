#![warn(warnings)]

#[macro_use]
mod ffi;

pub mod attach;
mod console;
mod container;
mod flags;
pub mod log;
mod migrate;

pub use self::container::Container;
pub use self::flags::{AttchFlags, CloneFlags, CreateFlags};
pub use self::log::Log;

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

    states.iter().map(|e| self::ffi::to_string(*e)).collect()
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
        Some(self::ffi::to_string(value))
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
 * Lists all active containers within the specified directory path.
 *
 * # Parameters
 *
 * * `path` - A reference to a string representing the directory path where the containers are located.
 *
 * # Return
 *
 * * `Result<Vec<String>>` - Returns a `Result` containing a vector of strings representing the names of the active containers.
 *   - `Ok(Vec<String>)`   - If the operation is successful, the vector contains the names of the active containers.
 *   - `Err(Error)`        - If an error occurs during the operation, an `Error` instance is returned.
 *
 * # Errors
 *
 * * If the function fails to list all active containers, it returns an `Error` with a description of the failure.
 *
 * # Safety
 *
 * This function uses unsafe Rust code to interact with the LXC library. It is important to ensure that the
 * provided `path` is a valid directory path and that the LXC library is properly initialized.
 */
pub fn list_active_containers(path: &str) -> Result<Vec<String>> {
    let mut containers = vec![];

    match unsafe {
        lxc_sys::list_active_containers(cstr!(path), std::ptr::null_mut(), std::ptr::null_mut())
            as i32
    } {
        error if error < 0 => {
            return Err(Error {
                num: 0,
                str: "Failed to list active containers".to_string(),
            });
        }
        size if size > 0 => {
            unsafe {
                let mut p_list: *mut *mut i8 = std::ptr::null_mut();

                lxc_sys::list_active_containers(cstr!(path), &mut p_list, std::ptr::null_mut());

                for i in 0..size {
                    let r: *mut i8 = *p_list.add(i as usize);
                    containers.push(self::ffi::to_string(r));

                    lxc_sys::free(r as *mut core::ffi::c_void);
                }
                lxc_sys::free(p_list as *mut core::ffi::c_void);
            };
        }
        _ => (), // No containers found
    }

    Ok(containers)
}

/**
 * Lists all containers within the specified directory path.
 *
 * # Parameters
 *
 * * `path` - A reference to a string representing the directory path where the containers are located.
 *
 * # Return
 *
 * * `Result<Vec<String>>` - Returns a `Result` containing a vector of strings representing the names of the containers.
 *   - `Ok(Vec<String>)`   - If the operation is successful, the vector contains the names of the containers.
 *   - `Err(Error)`        - If an error occurs during the operation, an `Error` instance is returned.
 *
 * # Errors
 *
 * * If the function fails to list all containers, it returns an `Error` with a description of the failure.
 *
 * # Safety
 *
 * This function uses unsafe Rust code to interact with the LXC library. It is important to ensure that the
 * provided `path` is a valid directory path and that the LXC library is properly initialized.
 */
pub fn list_all_containers(path: &str) -> Result<Vec<String>> {
    let mut containers = vec![];

    let size = unsafe {
        lxc_sys::list_all_containers(cstr!(path), std::ptr::null_mut(), std::ptr::null_mut())
    };

    if size < 0 {
        return Err(Error {
            num: size,
            str: "Failed to list all containers".to_string(),
        });
    }

    if size > 0 {
        unsafe {
            let mut p_list: *mut *mut i8 = std::ptr::null_mut();

            lxc_sys::list_all_containers(cstr!(path), &mut p_list, std::ptr::null_mut());

            for i in 0..size {
                let r: *mut i8 = *p_list.add(i as usize);
                containers.push(self::ffi::to_string(r));

                lxc_sys::free(r as *mut core::ffi::c_void);
            }
            lxc_sys::free(p_list as *mut core::ffi::c_void);
        };
    }

    Ok(containers)
}

/**
 * Lists all defined containers within the specified directory path.
 *
 * # Parameters
 *
 * * `path` - A reference to a string representing the directory path where the containers are located.
 *
 * # Return
 *
 * * `Result<Vec<String>>` - Returns a `Result` containing a vector of strings representing the names of the defined containers.
 *   - `Ok(Vec<String>)`   - If the operation is successful, the vector contains the names of the defined containers.
 *   - `Err(Error)`        - If an error occurs during the operation, an `Error` instance is returned.
 *
 * # Errors
 *
 * * If the function fails to list all containers, it returns an `Error` with a description of the failure.
 *
 * # Safety
 *
 * This function uses unsafe Rust code to interact with the LXC library. It is important to ensure that the
 * provided `path` is a valid directory path and that the LXC library is properly initialized.
 */
pub fn list_defined_containers(path: &str) -> Result<Vec<String>> {
    let mut containers = vec![];

    match unsafe {
        lxc_sys::list_defined_containers(cstr!(path), std::ptr::null_mut(), std::ptr::null_mut())
            as i32
    } {
        error if error < 0 => {
            return Err(Error {
                num: 0,
                str: "Failed to list defined containers".to_string(),
            });
        }
        size if size > 0 => {
            unsafe {
                let mut p_list: *mut *mut i8 = std::ptr::null_mut();

                lxc_sys::list_defined_containers(cstr!(path), &mut p_list, std::ptr::null_mut());

                for i in 0..size {
                    let r: *mut i8 = *p_list.add(i as usize);
                    containers.push(self::ffi::to_string(r));

                    lxc_sys::free(r as *mut core::ffi::c_void);
                }
                lxc_sys::free(p_list as *mut core::ffi::c_void);
            };
        }
        _ => (), // No containers found
    }

    Ok(containers)
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
pub fn get_lxc_path() -> Option<String> {
    get_global_config_item("lxc.lxcpath")
}
