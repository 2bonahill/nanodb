use anyhow::anyhow;
use serde::Serialize;
use serde_json::Value;
use std::{
    path::PathBuf,
    sync::{Arc, RwLock, RwLockReadGuard, RwLockWriteGuard},
};

use crate::{
    error::NanoDBError,
    tree::{PathStep, Tree},
};

/// A struct representing a NanoDB instance.
///
/// # Fields
///
/// * `path` - The path to the JSON file that this NanoDB instance is managing.
/// * `data` - The JSON data that this NanoDB instance is managing.
///
/// # Methods
///
/// * `new` - Synchronous constructor.
/// * `get` - Index into a JSON array or map.
/// * `insert` - Inserts a key-value pair into the JSON object.
/// * `write` - Write the current state of the JSON data to disk synchronously.
/// * `write_async` - Write the current state of the JSON data to disk asynchronously.
/// * `merge` - Pushes a value to a nested array specified by a string path.
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

    /// Index into a JSON array or map. A string index can be used to access a
    /// value in a map, and a usize index can be used to access an element of an
    /// array.
    ///
    /// Returns `None` if the type of `self` does not match the type of the
    /// index, for example if the index is a string and `self` is an array or a
    /// number. Also returns `None` if the given key does not exist in the map
    /// or the given index is not within the bounds of the array.
    pub fn get(&self, key: &str) -> Result<Tree, NanoDBError> {
        let data = self.data.read().map_err(|_| NanoDBError::RwLockReadError)?;
        let value = data
            .get(key)
            .ok_or_else(|| anyhow!("Key not found: {}", key))?;
        Ok(Tree {
            inner: value.clone(),
            path: vec![PathStep::Key(key.to_string())],
        })
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
    pub fn write(&mut self) -> Result<(), NanoDBError> {
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

    fn _write_lock(&mut self) -> Result<RwLockWriteGuard<'_, Value>, NanoDBError> {
        self.data.write().map_err(|_| NanoDBError::RwLockWriteError)
    }

    fn _read_lock(&mut self) -> Result<RwLockReadGuard<'_, Value>, NanoDBError> {
        self.data.read().map_err(|_| NanoDBError::RwLockReadError)
    }

    /// Pushes a value to a nested array specified by a string path.
    /// The path is a slice of strings, representing a sequence of keys (and indices represented as strings for arrays).
    pub fn merge<T, S>(&mut self, path: &[S], value: T) -> Result<(), NanoDBError>
    where
        T: Serialize,
        S: Into<String> + Clone,
    {
        let mut data = self
            .data
            .write()
            .map_err(|_| NanoDBError::RwLockWriteError)?;

        // Navigate to the target array using the path.
        let mut current = &mut *data;
        for p in path {
            let key = p.clone().into();

            // Attempt to treat the current value as an object and navigate to the next key.
            if current.is_object() {
                current = current
                    .get_mut(&key)
                    .ok_or_else(|| anyhow!("Key not found: {}", key))?;
            } else if let Ok(idx) = key.parse::<usize>() {
                // If the current value is not an object, try to parse the key as an array index.
                if !current.is_array() {
                    return Err(NanoDBError::NotAnArray);
                }
                let arr = current.as_array_mut().ok_or(NanoDBError::NotAnArray)?;
                current = arr.get_mut(idx).ok_or(NanoDBError::IndexOutOfBounds)?;
            } else {
                // If the key is not a valid index and we're not currently at an object, it's an error.
                return Err(NanoDBError::InvalidJSONPath);
            }
        }

        // Now `current` should be the target array.
        if let Some(arr) = current.as_array_mut() {
            dbg!(&arr);
            let serialized_value = serde_json::to_value(value)?;
            arr.push(serialized_value);
            dbg!(&arr);
            dbg!(&data);
            Ok(())
        } else {
            Err(NanoDBError::NotAnArray)
        }
    }
}
