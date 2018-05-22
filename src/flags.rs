bitflags! {
    pub struct CreateFlags: i32 {
        const QUIET = ::lxc_sys::LXC_CREATE_QUIET as i32;
        const MAXFLAGS = ::lxc_sys::LXC_CREATE_MAXFLAGS as i32;
    }
}
