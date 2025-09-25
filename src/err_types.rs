use std::fmt::Display;

#[derive(Debug)]
pub enum RustyDbErr {
    KeyNotFound(String),
    IoError(std::io::Error),
    SerializationError(String),
}

impl Display for RustyDbErr {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            RustyDbErr::KeyNotFound(key) => write!(f, "Key not found: {}", key),
            RustyDbErr::IoError(error) => write!(f, "IO error: {}", error),
            RustyDbErr::SerializationError(msg) => write!(f, "Serialization Error: {}", msg),
        }
    }
}

impl std::error::Error for RustyDbErr {}
