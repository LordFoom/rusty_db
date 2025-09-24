#[derive(Debug)]
pub enum RustyDbErr {
    KeyNotFound(String),
    IoError(std::io::Error),
    SerializationError(String),
}
