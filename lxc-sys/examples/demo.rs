use lxc_sys::lxc_container;
use std::ptr::{null, null_mut};

macro_rules! str {
    ($e:expr) => {{ std::ffi::CString::new($e).unwrap() }};
}

fn main() {
    unsafe {
        /* Setup container struct */
        let c = lxc_sys::lxc_container_new(str!("apicontainer").as_ptr(), null());

        if c == null_mut() {
            panic(c, "Failed to setup lxc_container struct");
        }

        if (*c).is_defined.unwrap()(c) {
            panic(c, "Container already exists");
        }

        /* Create the container */
        if !(*c).createl.unwrap()(
            c,
            str!("download").as_ptr(),
            null(),
            null_mut(),
            lxc_sys::LXC_CREATE_QUIET as i32,
            str!("-d").as_ptr(),
            str!("ubuntu").as_ptr(),
            str!("-r").as_ptr(),
            str!("trusty").as_ptr(),
            str!("-a").as_ptr(),
            str!("i386").as_ptr(),
            null() as *const i8,
        ) {
            panic(c, "Failed to create container rootfs");
        }

        /* Start the container */
        if !(*c).start.unwrap()(c, 0, null()) {
            panic(c, "Failed to start the container");
        }

        /* Query some information */
        let state = (*c).state.unwrap()(c);
        println!("Container state: {:?}", std::ffi::CStr::from_ptr(state));
        println!("Container PID: {}", (*c).init_pid.unwrap()(c));

        /* Stop the container */
        if !(*c).shutdown.unwrap()(c, 30) {
            println!("Failed to cleanly shutdown the container, forcing.");
            if !(*c).stop.unwrap()(c) {
                panic(c, "Failed to kill the container.");
            }
        }

        /* Destroy the container */
        if !(*c).destroy.unwrap()(c) {
            panic(c, "Failed to destroy the container.");
        }

        lxc_sys::lxc_container_put(c);
    }
}

unsafe fn panic(c: *mut lxc_container, message: &str) -> ! {
    lxc_sys::lxc_container_put(c);
    panic!("{}", message);
}
