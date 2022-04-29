use std::{io, result};

use derive_more::Display;
use yaml_rust::ScanError;

#[derive(Clone, PartialEq, Debug, Eq, Display)]
#[display(fmt = "{}", s)]
pub struct RHomeError {
    s: String,
}

impl RHomeError {
    pub fn new(s: String) -> Self {
        Self { s }
    }
}

impl From<io::Error> for RHomeError {
    fn from(e: io::Error) -> Self {
        RHomeError::new(e.to_string())
    }
}

impl From<ScanError> for RHomeError {
    fn from(e: ScanError) -> Self {
        RHomeError::new(e.to_string())
    }
}

pub type RHomeResult<T> = result::Result<T, RHomeError>;
