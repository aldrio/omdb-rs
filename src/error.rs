use std::error::Error as StdError;
use std::fmt;
use hyper::Error as HyperError;
use hyper::status::StatusCode;
use serde_json::Error as SerdeError;

#[derive(Debug)]
pub enum Error {
    /// An error originating from Hyper.
    Http(HyperError),
    /// An unexpected HTTP status code.
    Status(StatusCode),
    /// Error deserializing Api's JSON.
    Json(SerdeError),
    /// An error from OMDb.
    Api(String),

    Other(&'static str),
}

impl StdError for Error {
    fn description(&self) -> &str {
        match *self {
            Error::Http(ref err) => err.description(),
            Error::Status(status) => status.canonical_reason().unwrap_or("Unknown status"),
            Error::Json(ref err) => err.description(),
            Error::Api(ref desc) => desc.as_ref(),
            Error::Other(desc) => desc,
        }
    }

    fn cause(&self) -> Option<&StdError> {
        match *self {
            Error::Http(ref err) => Some(err),
            Error::Json(ref err) => Some(err),
            _ => None,
        }
    }
}

impl From<HyperError> for Error {
    fn from(err: HyperError) -> Error {
        Error::Http(err)
    }
}

impl From<SerdeError> for Error {
    fn from(err: SerdeError) -> Error {
        Error::Json(err)
    }
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            Error::Http(ref err) => err.fmt(f),
            Error::Json(ref err) => err.fmt(f),
            _ => f.write_str(self.description()),
        }
    }
}
