use std::{error, fmt, io, num, path, str};

#[derive(Debug)]
pub enum Error {
    EditorError(String),
    IoError(io::Error),
    FormatError,
    PathPrefixError,
    ParseBoolError,
    ParseIntError,
    RonError(ron::Error),
    TimeError(chrono::ParseError),
    UrlError(url::ParseError),
    OtherError(String),
}

impl From<io::Error> for Error {
    fn from(err: io::Error) -> Self {
        Self::IoError(err)
    }
}

impl From<fmt::Error> for Error {
    fn from(_: fmt::Error) -> Self {
        Self::FormatError
    }
}

impl From<str::ParseBoolError> for Error {
    fn from(_: str::ParseBoolError) -> Self {
        Self::ParseBoolError
    }
}

impl From<num::ParseIntError> for Error {
    fn from(_: num::ParseIntError) -> Self {
        Self::ParseIntError
    }
}

impl From<path::StripPrefixError> for Error {
    fn from(_: path::StripPrefixError) -> Self {
        Self::PathPrefixError
    }
}

impl From<ron::Error> for Error {
    fn from(err: ron::Error) -> Self {
        Self::RonError(err)
    }
}

impl From<chrono::ParseError> for Error {
    fn from(err: chrono::ParseError) -> Self {
        Self::TimeError(err)
    }
}

impl From<url::ParseError> for Error {
    fn from(err: url::ParseError) -> Self {
        Self::UrlError(err)
    }
}

impl From<String> for Error {
    fn from(s: String) -> Self {
        Self::OtherError(s)
    }
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::IoError(e) => write!(f, "{e}"),
            Self::FormatError => write!(f, "Format error"),
            Self::PathPrefixError => write!(f, "Path prefix error"),
            Self::ParseBoolError => write!(f, "Parse bool error"),
            Self::ParseIntError => write!(f, "Parse int error"),
            Self::RonError(e) => {
                write!(
                    f,
                    "Ron error code: {}\nposition:\n  line: {}\n  column: {}",
                    e.code, e.position.line, e.position.col,
                )
            }
            Self::TimeError(e) => write!(f, "{e}"),
            Self::UrlError(e) => write!(f, "{e}"),
            Self::EditorError(e) | Self::OtherError(e) => write!(f, "{e}"),
        }
    }
}

impl error::Error for Error {
    fn source(&self) -> Option<&(dyn error::Error + 'static)> {
        match self {
            Self::IoError(e) => Some(e),
            Self::FormatError => Some(&fmt::Error),
            Self::RonError(e) => Some(e),
            Self::TimeError(e) => Some(e),
            Self::UrlError(e) => Some(e),
            _ => None,
        }
    }
}
