use std::fmt::Display;

#[derive(Debug, PartialEq, PartialOrd)]
pub enum RustyDbErr {
    KeyNotFound(String),
    IoError(String),
    SerializationError(String),
}

impl Display for RustyDbErr {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            RustyDbErr::KeyNotFound(key) => write!(f, "Key not found: {}", key),
            RustyDbErr::IoError(err_msg) => {
                write!(f, "IO error - failed to save to disk {}", err_msg)
            }
            RustyDbErr::SerializationError(err_msg) => {
                write!(f, "Serialization Error: failed to encode data: {}", err_msg)
            }
        }
    }
}

impl std::error::Error for RustyDbErr {}

#[derive(Debug, PartialEq)]
pub enum ParseError {
    InvalidSyntax(String),
    MissingKeyWord(String),
}

impl std::fmt::Display for ParseError {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            ParseError::InvalidSyntax(msg) => write!(f, "Invalid Syntax: {}", msg),
            ParseError::MissingKeyWord(msg) => write!(f, "Missing Keyword: {}", msg),
        }
    }
}
