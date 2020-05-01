use crate::error::Error;
use crate::error_inner::ErrorInner;
use failure::Fail;
use serde::{Deserialize, Serialize};
use std::fmt;

#[derive(Debug, Fail, Serialize, Deserialize)]
pub struct IdError {
    pub id: String,
}

impl IdError {
    pub fn into_error(self) -> Error {
        Error {
            inner: Box::new(ErrorInner::IdError(self)),
        }
    }
}

impl fmt::Display for IdError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Could not find id: {}", self.id)
    }
}
