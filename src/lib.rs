extern crate lxc_sys;
#[macro_use]
extern crate bitflags;

mod flags;
mod container;

pub use self::flags::{AttchFlags, CloneFlags, CreateFlags};
pub use self::container::Container;

pub fn version() -> String {
    let version = unsafe {
        ::std::ffi::CStr::from_ptr(::lxc_sys::lxc_get_version())
    };

    version.to_str()
        .unwrap()
        .to_string()
}
