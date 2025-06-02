use std::{fmt, num::ParseIntError};

#[derive(Debug)]
pub enum Error {
    Generic(String),
    Conversion(String),
    Io(String),
    InvalidLength(String),
    InvalidNumber(String),
    Oob,
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
impl From<ParseIntError> for Error {
    fn from(value: ParseIntError) -> Self {
        Self::InvalidNumber(value.to_string())
    }
}

impl From<std::array::TryFromSliceError> for Error {
    fn from(value: std::array::TryFromSliceError) -> Self {
        Self::Conversion(value.to_string())
    }
}

impl From<std::io::Error> for Error {
    fn from(value: std::io::Error) -> Self {
        Self::Io(value.to_string())
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
            Self::InvalidNumber(message) => {
                write!(f, "Invalid Number: {}", message)
            }
            Self::Oob => {
                write!(f, "Index out of bound")
            }
            Self::Conversion(message) => {
                write!(f, "Conversion Error: {}", message)
            }
            Self::Io(message) => {
                write!(f, "I/O Error: {}", message)
            }
        }
    }
}

impl std::error::Error for Error {}
