use std::error;
use std::fmt;

#[derive(Debug, Clone)]
pub struct Error {
    e: ErrorType,
}

impl Error {
    pub fn new(t: ErrorType) -> Self {
        Error { e: t }
    }
}

#[derive(Debug, Clone)]
pub enum ErrorType {
    Parse,
    Connection,
    JSON,
    FileOpen,
    FileSave,
    Io,
    Json,
    Reqwest,
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match &self.e {
            ErrorType::Parse => write!(f, "Parse error"),
            ErrorType::Connection => write!(f, "Connection error"),
            ErrorType::JSON => write!(f, "JSON error"),
            ErrorType::FileOpen => write!(f, "File open error"),
            ErrorType::FileSave => write!(f, "File save error"),
            ErrorType::Io => write!(f, "IO error"),
            ErrorType::Json => write!(f, "JSON error"),
            ErrorType::Reqwest => write!(f, "Reqwest error"),
        }
    }
}

impl error::Error for Error {}

impl From<std::io::Error> for Error {
    fn from(_: std::io::Error) -> Self {
        Error::new(ErrorType::Io)
    }
}

impl From<serde_json::Error> for Error {
    fn from(_: serde_json::Error) -> Self {
        Error::new(ErrorType::Json)
    }
}

impl From<reqwest::Error> for Error {
    fn from(_: reqwest::Error) -> Self {
        Error::new(ErrorType::Reqwest)
    }
}
