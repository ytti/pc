use std::error::Error;
use std::fmt::{self, Display, Formatter};

#[derive(Debug)]
pub enum PasteError {
    Reqwest(reqwest::Error),
    Url(url::ParseError),
    IO(std::io::Error),
    ParseInt(std::num::ParseIntError),
    Message(String),
}

pub type PasteResult<T> = Result<T, PasteError>;

impl From<String> for PasteError {
    fn from(err: String) -> Self {
        PasteError::Message(err)
    }
}

impl From<reqwest::Error> for PasteError {
    fn from(err: reqwest::Error) -> Self {
        PasteError::Reqwest(err)
    }
}

impl From<url::ParseError> for PasteError {
    fn from(err: url::ParseError) -> Self {
        PasteError::Url(err)
    }
}

impl From<std::io::Error> for PasteError {
    fn from(err: std::io::Error) -> Self {
        PasteError::IO(err)
    }
}

impl From<std::num::ParseIntError> for PasteError {
    fn from(err: std::num::ParseIntError) -> Self {
        PasteError::ParseInt(err)
    }
}

impl Display for PasteError {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        write!(
            f,
            "{}",
            match self {
                PasteError::Reqwest(err) => format!("Request error: {}", err),
                PasteError::Url(err) => format!("Url error: {}", err),
                PasteError::IO(err) => format!("IO error: {}", err),
                PasteError::ParseInt(err) => format!("ParseInt error: {}", err),
                PasteError::Message(err) => format!("other error: {}", err),
            }
        )
    }
}

impl Error for PasteError {
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        match self {
            PasteError::Reqwest(err) => Some(err),
            PasteError::Url(err) => Some(err),
            PasteError::IO(err) => Some(err),
            PasteError::ParseInt(err) => Some(err),
            PasteError::Message(_) => None,
        }
    }
}
