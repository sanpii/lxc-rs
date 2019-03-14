mod level;

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

#[cfg(feature = "v2_0")]
impl std::convert::Into<lxc_sys::lxc_log> for Log {
    fn into(self) -> lxc_sys::lxc_log {
        let level: String = self.level.into();

        lxc_sys::lxc_log {
            name: to_cstr(&self.name),
            lxcpath: to_cstr(&self.lxcpath),
            file: to_cstr(&self.file),
            level: to_cstr(&level),
            prefix: to_cstr(&self.prefix),
            quiet: self.quiet,
        }
    }
}

impl Log {
    /**
     * Initialize the log.
     */
    pub fn init(self) -> Result<(), ()> {
        let success = unsafe {
            let success;
            #[cfg(feature = "v2_0")]
            {
                let mut info: lxc_sys::lxc_log = self.into();

                success = lxc_sys::lxc_log_init(&mut info);
            }
            #[cfg(not(feature = "v2_0"))]
            {
                success = lxc_sys::lxc_log_init(
                    to_cstr(&self.name),
                    to_cstr(&self.file),
                    self.level,
                    to_cstr(&self.prefix),
                    self.quiet,
                    to_cstr(&self.lxcpath),
                );
            }

            success
        };

        if success == 0 {
            Ok(())
        } else {
            Err(())
        }
    }

    /**
     * Close log file.
     */
    pub fn close() {
        unsafe {
            lxc_sys::lxc_log_close()
        }
    }
}
