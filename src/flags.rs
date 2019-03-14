#[cfg(feature = "v3_0")]
bitflags::bitflags! {
    pub struct AttchFlags: i32 {
        /** Move to cgroup. */
        const MOVE_TO_CGROUP = lxc_sys::LXC_ATTACH_MOVE_TO_CGROUP as i32;
        /** Drop capabilities. */
        const DROP_CAPABILITIES = lxc_sys::LXC_ATTACH_DROP_CAPABILITIES as i32;
        /** Set personality. */
        const SET_PERSONALITY = lxc_sys::LXC_ATTACH_SET_PERSONALITY as i32;
        /** Execute under a Linux Security Module. */
        const LSM_EXEC = lxc_sys::LXC_ATTACH_LSM_EXEC as i32;
        /** Remount `/pro`c filesystem. */
        const REMOUNT_PROC_SYS = lxc_sys::LXC_ATTACH_REMOUNT_PROC_SYS as i32;
        const LSM_NOW = lxc_sys::LXC_ATTACH_LSM_NOW as i32;
        const NO_NEW_PRIVS = lxc_sys::LXC_ATTACH_NO_NEW_PRIVS as i32;
        /** Allocate new terminal for attached process. */
        const TERMINAL = lxc_sys::LXC_ATTACH_TERMINAL as i32;
        /** Mask of flags to apply by default. */
        const DEFAULT = lxc_sys::LXC_ATTACH_DEFAULT as i32;
    }
}

#[cfg(not(feature = "v3_0"))]
bitflags::bitflags! {
    pub struct AttchFlags: i32 {
        /** Move to cgroup. */
        const MOVE_TO_CGROUP = lxc_sys::LXC_ATTACH_MOVE_TO_CGROUP as i32;
        /** Drop capabilities. */
        const DROP_CAPABILITIES = lxc_sys::LXC_ATTACH_DROP_CAPABILITIES as i32;
        /** Set personality. */
        const SET_PERSONALITY = lxc_sys::LXC_ATTACH_SET_PERSONALITY as i32;
        /** Execute under a Linux Security Module. */
        const LSM_EXEC = lxc_sys::LXC_ATTACH_LSM_EXEC as i32;
        /** Remount `/proc` filesystem. */
        const REMOUNT_PROC_SYS = lxc_sys::LXC_ATTACH_REMOUNT_PROC_SYS as i32;
        const LSM_NOW = lxc_sys::LXC_ATTACH_LSM_NOW as i32;
        /** Mask of flags to apply by default. */
        const DEFAULT = lxc_sys::LXC_ATTACH_DEFAULT as i32;
    }
}

bitflags::bitflags! {
    pub struct CloneFlags: i32 {
        /** Use the same bdev type. */
        const KEEPBDEVTYPE = lxc_sys::LXC_CLONE_KEEPBDEVTYPE as i32;
        /** Do not change the MAC address on network interfaces. */
        const KEEPMACADDR = lxc_sys::LXC_CLONE_KEEPMACADDR as i32;
        /** Do not edit the rootfs to change the hostname. */
        const KEEPNAME = lxc_sys::LXC_CLONE_KEEPNAME as i32;
        /** Number of `LXC_CLONE_*` flags. */
        const MAXFLAGS = lxc_sys::LXC_CLONE_MAXFLAGS as i32;
        /** Snapshot only if bdev supports it, else copy. */
        const MAYBE_SNAPSHOT = lxc_sys::LXC_CLONE_MAYBE_SNAPSHOT as i32;
        /** Snapshot the original filesystem(s). */
        const SNAPSHOT = lxc_sys::LXC_CLONE_SNAPSHOT as i32;
    }
}

bitflags::bitflags! {
    pub struct CreateFlags: i32 {
        /** Redirect stdin to `/dev/zero` and stdout and stderr to `/dev/null`. */
        const QUIET = lxc_sys::LXC_CREATE_QUIET as i32;
        /** Number of `LXC_CREATE*` flags. */
        const MAXFLAGS = lxc_sys::LXC_CREATE_MAXFLAGS as i32;
    }
}
