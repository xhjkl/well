//! Descriptions of what could go wrong.
use std::fmt;

pub struct Error(pub String);

impl From<&str> for Error {
    fn from(s: &str) -> Self {
        Self(s.to_owned())
    }
}

impl From<String> for Error {
    fn from(s: String) -> Self {
        Self(s)
    }
}

impl fmt::Debug for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(&self.0)
    }
}

pub struct Exit(Result<(), Error>);

impl From<Result<(), Error>> for Exit {
    fn from(result: Result<(), Error>) -> Self {
        Self(result)
    }
}
