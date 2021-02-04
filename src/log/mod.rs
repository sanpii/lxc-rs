mod level;

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
            name: cstr!(&self.name),
            lxcpath: cstr!(&self.lxcpath),
            file: cstr!(&self.file),
            level: cstr!(&level),
            prefix: cstr!(&self.prefix),
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

    #[cfg(feature = "v2_1")]
    fn log_init(self) -> i32 {
        let mut info: lxc_sys::lxc_log = self.into();

        unsafe { lxc_sys::lxc_log_init(&mut info) }
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
