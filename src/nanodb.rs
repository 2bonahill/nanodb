use anyhow::anyhow;
use serde::{Deserialize, Serialize};
use serde_json::Value;
use std::{
    path::PathBuf,
    sync::{Arc, RwLock},
};

use crate::error::NanoDBError;

#[derive(Debug)]
pub struct NanoDB {
    path: PathBuf,
    data: Arc<RwLock<Value>>,
}
impl NanoDB {
    /// Synchronous constructor
    pub fn new(path: impl Into<PathBuf>) -> Result<Self, NanoDBError> {
        let path = path.into();
        let data = if path.exists() {
            let contents = std::fs::read_to_string(&path)?;
            serde_json::from_str(&contents)?
        } else {
            Value::Object(Default::default())
        };

        Ok(Self {
            path,
            data: Arc::new(RwLock::new(data)),
        })
    }

    pub fn get<T: for<'de> Deserialize<'de>>(&self, key: &str) -> Result<T, NanoDBError> {
        let data = self.data.read().map_err(|_| NanoDBError::RwLockReadError)?;
        let value = data
            .get(key)
            .ok_or_else(|| anyhow!("Key not found: {}", key))?;
        serde_json::from_value(value.clone()).map_err(Into::into)
    }

    /// Inserts a key-value pair into the JSON object.
    /// If the JSON object did not have this key present, None is returned.
    /// If the JSON object did have this key present, the value is updated, and the old value is returned.
    /// The key is not updated
    pub fn insert<T: Serialize>(&mut self, key: &str, value: T) -> Result<(), NanoDBError> {
        let mut data = self
            .data
            .write()
            .map_err(|_| NanoDBError::RwLockReadError)?;
        let value = serde_json::to_value(value)?;
        data.as_object_mut().unwrap().insert(key.to_string(), value);
        Ok(())
    }

    /// Write the current state of the JSON data to disk asynchronously
    pub fn write(&self) -> Result<(), NanoDBError> {
        let data_guard = self
            .data
            .write()
            .map_err(|e| anyhow!("Failed to acquire lock: {}", e))?;
        let contents = serde_json::to_string_pretty(&*data_guard)?;
        std::fs::write(&self.path, contents)?;
        Ok(())
    }

    /// Write the current state of the JSON data to disk asynchronously
    pub async fn write_async(&self) -> Result<(), NanoDBError> {
        let data_guard = self
            .data
            .write()
            .map_err(|e| anyhow!("Failed to acquire lock: {}", e))?;
        let contents = serde_json::to_string_pretty(&*data_guard)?;
        tokio::fs::write(&self.path, contents).await?;
        Ok(())
    }
}
