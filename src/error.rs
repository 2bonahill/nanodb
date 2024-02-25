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
    #[error("RwLock Read error")]
    RwLockReadError,
    #[error("RwLock Write error")]
    RwLockWriteError,
    // Vasic error from anyhow
    #[error("Error: {0}")]
    Anyhow(#[from] anyhow::Error),
}
