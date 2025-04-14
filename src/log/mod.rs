mod level;

pub use level::Level;

pub struct Log {
    pub name: String,
    pub lxcpath: String,
    pub file: String,
    pub level: Level,
    pub prefix: String,
    pub quiet: bool,
}

#[cfg(feature = "v2_1")]
impl From<Log> for lxc_sys::lxc_log {
    fn from(value: Log) -> Self {
        let level: String = value.level.into();

        Self {
            name: cstr!(&value.name),
            lxcpath: cstr!(&value.lxcpath),
            file: cstr!(&value.file),
            level: cstr!(&level),
            prefix: cstr!(&value.prefix),
            quiet: value.quiet,
        }
    }
}

impl Log {
    /**
     * Initialize the log.
     */
    pub fn init(self) -> crate::Result {
        match self.log_init() {
            0 => Ok(()),
            num => Err(crate::Error {
                num,
                str: "Fail to initialize log".to_string(),
            }),
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
