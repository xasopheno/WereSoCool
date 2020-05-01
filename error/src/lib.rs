pub mod error;
pub mod error_inner;
pub mod id_error;
pub mod index_error;
pub mod parse_error;
pub mod portaudio_error;

pub use error::Error;
pub use error_inner::ErrorInner;
pub use id_error::IdError;
pub use index_error::IndexError;
pub use parse_error::ParseError;
use portaudio_error::PortAudioError;
use serde::{Deserialize, Serialize};
use std::io;

#[derive(Debug, Serialize, Deserialize)]
pub enum Serializable {
    Msg(String),
    IoError(String),
    #[serde(with = "PortAudioError")]
    PortAudio(portaudio::error::Error),
    SerdeJsonError(String),
    CSVError(String),
    ParseError(ParseError),
    IdError(IdError),
    IndexError(IndexError),
}

impl ErrorInner {
    pub fn into_serializeable(self) -> Serializable {
        match self {
            ErrorInner::Msg(e) => Serializable::Msg(e),
            ErrorInner::ParseError(e) => Serializable::ParseError(e),
            ErrorInner::IdError(e) => Serializable::IdError(e),
            ErrorInner::IndexError(e) => Serializable::IndexError(e),
            ErrorInner::Io(e) => {
                println!("{:#?}", e);
                Serializable::IoError("".to_string())
            }
            ErrorInner::SerdeJson(e) => {
                println!("{:#?}", e);
                Serializable::SerdeJsonError("SerdeJson Error".to_string())
            }
            ErrorInner::CSVError(e) => {
                println!("{:#?}", e);
                Serializable::CSVError("CSVError".to_string())
            }
            _ => unimplemented!(),
        }
    }
}

impl<'a> From<&'a str> for Error {
    fn from(msg: &'a str) -> Error {
        Error::with_msg(msg)
    }
}

impl From<IdError> for Error {
    fn from(e: IdError) -> Error {
        Error {
            inner: Box::new(ErrorInner::IdError(e)),
        }
    }
}

impl From<io::Error> for Error {
    fn from(e: io::Error) -> Error {
        Error {
            inner: Box::new(ErrorInner::Io(e)),
        }
    }
}

impl From<portaudio::error::Error> for Error {
    fn from(e: portaudio::error::Error) -> Error {
        Error {
            inner: Box::new(ErrorInner::PortAudio(e)),
        }
    }
}

impl From<serde_json::error::Error> for Error {
    fn from(e: serde_json::error::Error) -> Error {
        Error {
            inner: Box::new(ErrorInner::SerdeJson(e)),
        }
    }
}

impl From<csv::Error> for Error {
    fn from(e: csv::Error) -> Error {
        Error {
            inner: Box::new(ErrorInner::CSVError(e)),
        }
    }
}

impl From<ParseError> for Error {
    fn from(e: ParseError) -> Error {
        Error {
            inner: Box::new(ErrorInner::ParseError(e)),
        }
    }
}

#[test]
fn size_of_error_is_one_word() {
    use std::mem;
    assert_eq!(mem::size_of::<Error>(), mem::size_of::<usize>());
}
