#![allow(improper_ctypes)]
#![allow(non_upper_case_globals)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![allow(clippy::redundant_static_lifetimes)]
#![allow(clippy::unreadable_literal)]

include!(concat!(env!("OUT_DIR"), "/bindings.rs"));

impl Default for lxc_attach_options_t {
    fn default() -> Self {
        Self {
            attach_flags: LXC_ATTACH_DEFAULT as i32,
            namespaces: -1,
            personality: LXC_ATTACH_DETECT_PERSONALITY as i64,
            initial_cwd: std::ptr::null_mut(),
            gid: u32::MAX,
            uid: u32::MAX,
            env_policy: lxc_attach_env_policy_t_LXC_ATTACH_KEEP_ENV,
            extra_env_vars: std::ptr::null_mut(),
            extra_keep_env: std::ptr::null_mut(),
            stdout_fd: 0,
            stdin_fd: 1,
            stderr_fd: 2,
            #[cfg(feature = "v3_0")]
            log_fd: -libc::EBADF,
            #[cfg(feature = "v4_0")]
            lsm_label: std::ptr::null_mut(),
            #[cfg(feature = "v4_0")]
            groups: lxc_groups_t {
                size: 0,
                list: std::ptr::null_mut(),
            },
        }
    }
}
