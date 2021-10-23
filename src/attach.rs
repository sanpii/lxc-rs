/** LXC attach function type. */
pub use lxc_sys::lxc_attach_exec_t as ExecFn;

/** LXC attach options for lxc::Container::attach(). */
pub use lxc_sys::lxc_attach_options_t as Options;

/**
 * Run a shell command in the container.
 *
 * `payload` parameter is not used.
 *
 * Returns exit code of shell.
 */
pub use lxc_sys::lxc_attach_run_shell as run_shell;

/**
 * Run a command in the container.
 *
 * Returns exit code program on success.
 */
pub use lxc_sys::lxc_attach_run_command as run_command;

bitflags::bitflags! {
    /** LXC environment policy. */
    pub struct EnvPolicy: i32 {
        /** Retain the environment */
        const KEEP_ENV = lxc_sys::lxc_attach_env_policy_t_LXC_ATTACH_KEEP_ENV as i32;
        /** Clear the environment */
        const CLEAR_ENV = lxc_sys::lxc_attach_env_policy_t_LXC_ATTACH_CLEAR_ENV as i32;
    }
}
