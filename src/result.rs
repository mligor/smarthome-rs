use std::{io, result};

use derive_more::Display;
use yaml_rust::ScanError;

#[derive(Clone, PartialEq, Debug, Eq, Display)]
#[display(fmt = "{}", s)]
pub struct Error {
    s: String,
}

impl Error {
    pub fn new(s: String) -> Self {
        Self { s }
    }
}

impl From<io::Error> for Error {
    fn from(e: io::Error) -> Self {
        Error::new(e.to_string())
    }
}

impl From<ScanError> for Error {
    fn from(e: ScanError) -> Self {
        Error::new(e.to_string())
    }
}

pub type Result<T> = result::Result<T, Error>;
