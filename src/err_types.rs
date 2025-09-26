use std::fmt::Display;

#[derive(Debug)]
pub enum RustyDbErr {
    KeyNotFound(String),
    IoError(std::io::Error),
    SerializationError(&str, &str),
}

impl Display for RustyDbErr {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            RustyDbErr::KeyNotFound(key) => write!(f, "Key not found: {}", key),
            RustyDbErr::IoError(error) => write!(f, "IO error: {}", error),
            RustyDbErr::SerializationError(key, val) => {
                write!(f, "Serialization Error: failed to insert {}:{}", key, val)
            }
        }
    }
}

impl std::error::Error for RustyDbErr {}
