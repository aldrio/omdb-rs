use std::error::Error as StdError;
use std::fmt;

use reqwest::StatusCode;

#[derive(Debug)]
pub enum Error {
    /// An error originating from Reqwest.
    Http(reqwest::Error),
    /// An unexpected HTTP status code.
    Status(StatusCode),
    /// An error from OMDb.
    Api(String),

    Other(&'static str),
}

impl StdError for Error {
    fn description(&self) -> &str {
        match *self {
            Error::Http(ref err) => err.description(),
            Error::Status(status) => status.canonical_reason().unwrap_or("Unknown status"),
            Error::Api(ref desc) => desc.as_ref(),
            Error::Other(desc) => desc,
        }
    }

    fn cause(&self) -> Option<&StdError> {
        match *self {
            Error::Http(ref err) => Some(err),
            _ => None,
        }
    }
}

impl From<reqwest::Error> for Error {
    fn from(err: reqwest::Error) -> Error {
        Error::Http(err)
    }
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            Error::Http(ref err) => err.fmt(f),
            _ => f.write_str(self.description()),
        }
    }
}
