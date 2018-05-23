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
    pub fn init(self) -> Result<(), ()> {
        let mut info: ::lxc_sys::lxc_log = self.into();

        let success = unsafe {
            ::lxc_sys::lxc_log_init(&mut info)
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

