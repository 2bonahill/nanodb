use thiserror::Error;

#[derive(Error, Debug)]
pub enum NanoDBError {
    // Serde serialize error
    #[error("Serde deserialize error: {0}")]
    DeserializeFromStr(#[from] serde_json::Error),
    // IO error
    #[error("IO error: {0}")]
    Io(#[from] std::io::Error),
    // Lock error
    #[error("RwLock Read error: {0}")]
    RwLockReadError(String),
    #[error("RwLock Write error: {0}")]
    RwLockWriteError(String),
    #[error("The value at '{0}' is not array")]
    NotAnArray(String),
    #[error("The value at '{0}' does not have a length property")]
    LenNotDefined(String),
    #[error("The value at '{0}' is not an object")]
    NotAnObject(String),
    #[error("Key '{0}' not found")]
    KeyNotFound(String),
    #[error("Index {0} is out of bounds")]
    IndexOutOfBounds(usize),
    #[error("Invalid JSON path")]
    InvalidJSONPath,
    #[error("Type mismatch: {0}")]
    TypeMismatch(String),
    // Default error
    #[error("An error occurred")]
    DefaultError,
}
