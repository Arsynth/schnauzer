use std::fmt;

#[derive(Debug)]
pub enum Error {
    BadMagic(u32),
    BadBufferLength,
    Other(Box<dyn std::error::Error>),
}

impl Error {
    fn description(&self) -> String {
        match self {
            Error::BadMagic(v) => format!("Unknown magic: {v}"),
            Error::BadBufferLength => format!("Invalid buffer length"),
            Error::Other(e) => format!("Internal error: {:#?}", e),
        }
    }
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}

impl std::error::Error for Error {}

impl From<scroll::Error> for Error {
    fn from(value: scroll::Error) -> Self {
        Error::Other(Box::new(value))
    }
}

impl From<Error> for scroll::Error {
    fn from(value: Error) -> Self {
        scroll::Error::Custom(value.description())
    }
}

impl From<std::io::Error> for Error {
    fn from(value: std::io::Error) -> Self {
        Error::Other(Box::new(value))
    }
}

unsafe impl Send for Error {}
unsafe impl Sync for Error {}

pub type Result<T> = std::result::Result<T, Error>;
