use std::convert::{From, Into};

/// An error.
#[derive(Debug)]
pub struct Error {
    pub message: Option<String>,
}

impl<T> From<T> for Error where T: Into<String> {
    #[inline]
    fn from(message: T) -> Error {
        Error {
            message: Some(message.into()),
        }
    }
}
