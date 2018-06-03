pub use ::lxc_sys::lxc_attach_exec_t as ExecFn;
pub use ::lxc_sys::lxc_attach_options_t as Options;

bitflags! {
    pub struct EnvPolicy: i32 {
        const KEEP_ENV = ::lxc_sys::lxc_attach_env_policy_t_LXC_ATTACH_KEEP_ENV as i32;
        const CLEAR_ENV = ::lxc_sys::lxc_attach_env_policy_t_LXC_ATTACH_CLEAR_ENV as i32;
    }
}

pub fn run_command(payload: &mut ::std::os::raw::c_void) -> i32
{
    unsafe {
        ::lxc_sys::lxc_attach_run_command(payload)
    }
}

pub fn run_shell(payload: &mut ::std::os::raw::c_void) -> i32
{
    unsafe {
        ::lxc_sys::lxc_attach_run_shell(payload)
    }
}
