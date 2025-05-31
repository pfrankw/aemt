use std::fmt;

#[derive(Debug)]
pub enum Error {
    Generic(String),
    InvalidLength(String),
}

impl From<String> for Error {
    fn from(value: String) -> Self {
        Self::Generic(value)
    }
}

impl From<&str> for Error {
    fn from(value: &str) -> Self {
        Self::Generic(value.to_string())
    }
}

impl From<std::array::TryFromSliceError> for Error {
    fn from(value: std::array::TryFromSliceError) -> Self {
        Self::Generic(value.to_string())
    }
}

impl From<std::io::Error> for Error {
    fn from(value: std::io::Error) -> Self {
        Self::Generic(value.to_string())
    }
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Generic(message) => {
                write!(f, "Generic error: {}", message)
            }
            Self::InvalidLength(message) => {
                write!(f, "Invalid Length: {}", message)
            }
        }
    }
}

impl std::error::Error for Error {}
