pub enum Level {
    Trace,
    Debug,
    Info,
    Notice,
    Warn,
    Error,
    Crit,
    Alert,
    Fatal,
    Notset,
}

impl ::std::convert::Into<String> for Level {
    fn into(self) -> String {
        use self::Level::*;

        let s = match self {
            Trace => "TRACE",
            Debug => "DEBUG",
            Info => "INFO",
            Notice => "NOTICE",
            Warn => "WARN",
            Error => "ERROR",
            Crit => "CRIT",
            Alert => "ALERT",
            Fatal => "FATAL",
            Notset => "NOTSET",
        };

        s.to_string()
    }
}
