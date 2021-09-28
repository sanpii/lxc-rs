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
impl std::convert::TryInto<lxc_sys::lxc_log> for Log {
    type Error = crate::Error;

    fn try_into(self) -> Result<lxc_sys::lxc_log, Self::Error> {
        let level: String = self.level.into();

        let log = lxc_sys::lxc_log {
            name: cstr!(&self.name),
            lxcpath: cstr!(&self.lxcpath),
            file: cstr!(&self.file),
            level: cstr!(&level),
            prefix: cstr!(&self.prefix),
            quiet: self.quiet,
        };

        Ok(log)
    }
}

impl Log {
    /**
     * Initialize the log.
     */
    pub fn init(self) -> crate::Result {
        if self.log_init()? == 0 {
            Ok(())
        } else {
            Err(crate::Error::Unknow)
        }
    }

    #[cfg(not(feature = "v2_1"))]
    fn log_init(self) -> crate::Result<i32> {
        Ok(-1)
    }

    #[cfg(feature = "v2_1")]
    fn log_init(self) -> crate::Result<i32> {
        use std::convert::TryInto;

        let mut info: lxc_sys::lxc_log = self.try_into()?;

        let log = unsafe { lxc_sys::lxc_log_init(&mut info) };

        Ok(log)
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
