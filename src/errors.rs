pub type Result<T = ()> = std::result::Result<T, Error>;

#[derive(Debug, thiserror::Error)]
pub enum Error {
    #[error(transparent)]
    FromUtf8(#[from] std::string::FromUtf8Error),
    #[error(transparent)]
    InvalidNetworkAddress(#[from] std::net::AddrParseError),
    #[error("{message}")]
    LXCContainer { num: i32, message: String },
    #[error(transparent)]
    Nul(#[from] std::ffi::NulError),
    #[error("Unavailable function {0} for this container")]
    UnavailableFunction(String),
    #[error(transparent)]
    Utf8(#[from] std::str::Utf8Error),
    #[error("unknow error")]
    Unknow,
}
