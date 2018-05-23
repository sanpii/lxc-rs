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

#[cfg(feature = "v2_0")]
impl ::std::convert::Into<::lxc_sys::lxc_log> for Log {
    fn into(self) -> ::lxc_sys::lxc_log {
        let level: String = self.level.into();

        ::lxc_sys::lxc_log {
            name: string!(self.name),
            lxcpath: string!(self.lxcpath),
            file: string!(self.file),
            level: string!(level),
            prefix: string!(self.prefix),
            quiet: self.quiet,
        }
    }
}

impl Log {
    #[cfg(feature = "v2_0")]
    pub fn init(self) -> Result<(), ()> {
        let success = unsafe {
            let mut success;

            #[cfg(feature = "v2_0")]
            {
                let mut info: ::lxc_sys::lxc_log = self.into();
                success = ::lxc_sys::lxc_log_init(&mut info);
            }
            #[cfg(not(feature = "v2_0"))]
            success = ::lxc_sys::lxc_log_init(string!(info.name), string!(info.file), info.level.into(), string!(info.prefix), info.quiet, string!(info.lxcpath));

            success
        };

        if success == 0 {
            Ok(())
        } else {
            Err(())
        }
    }

    pub fn close() {
        unsafe {
            ::lxc_sys::lxc_log_close()
        }
    }
}

