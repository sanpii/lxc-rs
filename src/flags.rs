#[cfg(feature = "v3_0")]
bitflags! {
    pub struct AttchFlags: i32 {
        const MOVE_TO_CGROUP = ::lxc_sys::LXC_ATTACH_MOVE_TO_CGROUP as i32;
        const DROP_CAPABILITIES = ::lxc_sys::LXC_ATTACH_DROP_CAPABILITIES as i32;
        const SET_PERSONALITY = ::lxc_sys::LXC_ATTACH_SET_PERSONALITY as i32;
        const LSM_EXEC = ::lxc_sys::LXC_ATTACH_LSM_EXEC as i32;
        const REMOUNT_PROC_SYS = ::lxc_sys::LXC_ATTACH_REMOUNT_PROC_SYS as i32;
        const LSM_NOW = ::lxc_sys::LXC_ATTACH_LSM_NOW as i32;
        const NO_NEW_PRIVS = ::lxc_sys::LXC_ATTACH_NO_NEW_PRIVS as i32;
        const TERMINAL = ::lxc_sys::LXC_ATTACH_TERMINAL as i32;
        const DEFAULT = ::lxc_sys::LXC_ATTACH_DEFAULT as i32;
    }
}

#[cfg(not(feature = "v3_0"))]
bitflags! {
    pub struct AttchFlags: i32 {
        const MOVE_TO_CGROUP = ::lxc_sys::LXC_ATTACH_MOVE_TO_CGROUP as i32;
        const DROP_CAPABILITIES = ::lxc_sys::LXC_ATTACH_DROP_CAPABILITIES as i32;
        const SET_PERSONALITY = ::lxc_sys::LXC_ATTACH_SET_PERSONALITY as i32;
        const LSM_EXEC = ::lxc_sys::LXC_ATTACH_LSM_EXEC as i32;
        const REMOUNT_PROC_SYS = ::lxc_sys::LXC_ATTACH_REMOUNT_PROC_SYS as i32;
        const LSM_NOW = ::lxc_sys::LXC_ATTACH_LSM_NOW as i32;
        const DEFAULT = ::lxc_sys::LXC_ATTACH_DEFAULT as i32;
    }
}

bitflags! {
    pub struct CloneFlags: i32 {
        const KEEPBDEVTYPE = ::lxc_sys::LXC_CLONE_KEEPBDEVTYPE as i32;
        const KEEPMACADDR = ::lxc_sys::LXC_CLONE_KEEPMACADDR as i32;
        const KEEPNAME = ::lxc_sys::LXC_CLONE_KEEPNAME as i32;
        const MAXFLAGS = ::lxc_sys::LXC_CLONE_MAXFLAGS as i32;
        const MAYBE_SNAPSHOT = ::lxc_sys::LXC_CLONE_MAYBE_SNAPSHOT as i32;
        const SNAPSHOT = ::lxc_sys::LXC_CLONE_SNAPSHOT as i32;
    }
}

bitflags! {
    pub struct CreateFlags: i32 {
        const QUIET = ::lxc_sys::LXC_CREATE_QUIET as i32;
        const MAXFLAGS = ::lxc_sys::LXC_CREATE_MAXFLAGS as i32;
    }
}
