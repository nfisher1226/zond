use std::{error, fmt, io, num, path, str};

#[derive(Debug)]
/// Zond errors
pub enum Error {
    /// An error opening the page in your editor
    EditorError(String),
    /// An error reading or writing from disk
    IoError(io::Error),
    /// An error while formatting data
    FormatError,
    /// An error with an incorrect path prefix
    PathPrefixError,
    /// An error parsing a boolean from the command line
    ParseBoolError,
    /// An error parsing an integer from the command line
    ParseIntError,
    /// An error parsing an enum type from a string slice
    ParseEnumError,
    /// An error reading or writing the configuration file
    RonError(ron::Error),
    /// An error parsing the time of publication
    TimeError(chrono::ParseError),
    /// An error parsing a url
    UrlError(url::ParseError),
    /// Another, unexpected, error
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

impl From<ron::error::SpannedError> for Error {
    fn from(err: ron::error::SpannedError) -> Self {
        Self::RonError(err.into())
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
            Self::ParseEnumError => write!(f, "Parse enum error"),
            Self::RonError(e) => write!(f, "Ron error: {e}"),
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
