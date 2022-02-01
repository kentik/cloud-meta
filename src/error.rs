use std::fmt;
use http::{StatusCode, uri::{InvalidUri, InvalidUriParts}};

#[derive(Debug, Eq, PartialEq)]
pub enum Error {
    Response(StatusCode),
    Internal(String),
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:?}", self)
    }
}

impl std::error::Error for Error {}

impl From<std::io::Error> for Error {
    fn from(err: std::io::Error) -> Self {
        Self::Internal(err.to_string())
    }
}

impl From<std::string::FromUtf8Error> for Error {
    fn from(err: std::string::FromUtf8Error) -> Self {
        Self::Internal(err.to_string())
    }
}

impl From<StatusCode> for Error {
    fn from(err: StatusCode) -> Self {
        Self::Response(err)
    }
}

impl From<InvalidUri> for Error {
    fn from(err: InvalidUri) -> Self {
        Self::Internal(err.to_string())
    }
}

impl From<InvalidUriParts> for Error {
    fn from(err: InvalidUriParts) -> Self {
        Self::Internal(err.to_string())
    }
}

impl From<hyper::Error> for Error {
    fn from(err: hyper::Error) -> Self {
        Self::Internal(err.to_string())
    }
}

impl From<hyper::header::InvalidHeaderValue> for Error {
    fn from(err: hyper::header::InvalidHeaderValue) -> Self {
        Self::Internal(err.to_string())
    }
}

impl From<serde_json::Error> for Error {
    fn from(err: serde_json::Error) -> Self {
        Self::Internal(err.to_string())
    }
}
