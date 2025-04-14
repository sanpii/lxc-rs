macro_rules! list_containers {
    ($name:ident, $fn:ident) => {
        #[doc = concat!("Lists ", stringify!($name), " containers within the specified directory path.")]
        /**
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
         * * If the function fails to list containers, it returns an `Error` with a description of the failure.
         *
         * # Safety
         *
         * This function uses unsafe Rust code to interact with the LXC library. It is important to ensure that the
         * provided `path` is a valid directory path and that the LXC library is properly initialized.
         */
        pub fn $name(path: &str) -> crate::Result<Vec<String>> {
            let mut names = std::ptr::null_mut();

            let size = unsafe {
                lxc_sys::$fn(cstr!(path), &mut names, std::ptr::null_mut())
            };

            if size < 0 {
                return Err(crate::Error {
                    num: 0,
                    str: format!("Failed to list {} containers", stringify!($name)),
                });
            }

            let containers = crate::ffi::vec_from_nta(names)
                .into_iter()
                .map(|x| crate::ffi::to_string(x))
                .collect();

            unsafe {
                lxc_sys::free(names as *mut std::ffi::c_void);
            }

            Ok(containers)
        }
    }
}

list_containers!(active, list_active_containers);
list_containers!(all, list_all_containers);
list_containers!(defined, list_defined_containers);
