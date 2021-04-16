use crate::{Error, ErrorInner};
use serde::{Deserialize, Serialize};
use std::fmt;
use thiserror::Error;

#[derive(Debug, Error, Serialize, Deserialize)]
pub struct IndexError {
    pub len_list: usize,
    pub index: i64,
    pub message: String,
}

impl IndexError {
    pub fn into_error(self) -> Error {
        Error {
            inner: Box::new(ErrorInner::IndexError(self)),
        }
    }
}

impl fmt::Display for IndexError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.message)
    }
}

impl From<IndexError> for Error {
    fn from(e: IndexError) -> Error {
        Error {
            inner: Box::new(ErrorInner::IndexError(e)),
        }
    }
}
