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

impl From<Level> for String {
    fn from(level: Level) -> Self {
        use self::Level::*;

        let s = match level {
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
