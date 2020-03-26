mod level;

#[cfg(feature = "v2_1")]
use crate::ffi::to_cstr;

pub use self::level::Level;

pub struct Log {
    pub name: String,
    pub lxcpath: String,
    pub file: String,
    pub level: self::Level,
    pub prefix: String,
    pub quiet: bool,
}

#[cfg(feature = "v2_1")]
impl std::convert::Into<lxc_sys::lxc_log> for Log {
    fn into(self) -> lxc_sys::lxc_log {
        let level: String = self.level.into();

        lxc_sys::lxc_log {
            name: to_cstr(&self.name).as_ptr(),
            lxcpath: to_cstr(&self.lxcpath).as_ptr(),
            file: to_cstr(&self.file).as_ptr(),
            level: to_cstr(&level).as_ptr(),
            prefix: to_cstr(&self.prefix).as_ptr(),
            quiet: self.quiet,
        }
    }
}

impl Log {
    /**
     * Initialize the log.
     */
    pub fn init(self) -> Result<(), ()> {
        if self.log_init() == 0 {
            Ok(())
        } else {
            Err(())
        }
    }

    #[cfg(not(feature = "v2_1"))]
    fn log_init(self) -> i32 {
        -1
    }

    #[cfg(any(all(feature = "v2_1", not(feature = "v3_0")), feature = "v3_1"))]
    fn log_init(self) -> i32 {
        let mut info: lxc_sys::lxc_log = self.into();

        unsafe { lxc_sys::lxc_log_init(&mut info) }
    }

    #[cfg(all(feature = "v3_0", not(feature = "v3_1")))]
    fn log_init(self) -> i32 {
        unsafe {
            lxc_sys::lxc_log_init(
                to_cstr(&self.name),
                to_cstr(&self.file),
                self.level,
                to_cstr(&self.prefix),
                self.quiet,
                to_cstr(&self.lxcpath),
            )
        }
    }

    /**
     * Close log file.
     */
    pub fn close() {
        #[cfg(feature = "v2_0")]
        unsafe {
            lxc_sys::lxc_log_close()
        }
    }
}
