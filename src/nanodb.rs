use serde::Serialize;
use serde_json::Value;
use std::{
    path::PathBuf,
    sync::{Arc, RwLock, RwLockReadGuard, RwLockWriteGuard},
};
use tempfile::tempdir;

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
    /// Creates a new NanoDB instance with the JSON data from the file at the given path.
    ///
    /// If the file does not exist, the NanoDB instance is initialized with an empty JSON object.
    ///
    /// # Arguments
    ///
    /// * `path` - The path to the JSON file. This argument is converted into a `PathBuf`.
    ///
    /// # Returns
    ///
    /// * `Ok(NanoDB)` - A new NanoDB instance with the JSON data from the file at `path`.
    /// * `Err(NanoDBError::FileReadError)` - If there was an error reading the file.
    /// * `Err(serde_json::Error)` - If there was an error parsing the file contents as JSON.
    ///
    /// # Examples
    ///
    /// ```text
    /// let db = NanoDB::open("path/to/json/file.json").unwrap();
    /// ```
    pub fn open(path: impl Into<PathBuf>) -> Result<Self, NanoDBError> {
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

    /// Creates a new NanoDB instance with the given JSON data and writes it to the file at the given path.
    ///
    /// If the file does not exist, it is created.
    ///
    /// # Arguments
    ///
    /// * `path` - The path to the JSON file. This argument is converted into a `PathBuf`.
    /// * `contents` - The JSON data to initialize the NanoDB instance with and write to the file.
    ///
    /// # Returns
    ///
    /// * `Ok(NanoDB)` - A new NanoDB instance with the given JSON data.
    /// * `Err(NanoDBError::FileWriteError)` - If there was an error writing to the file.
    /// * `Err(serde_json::Error)` - If there was an error parsing `contents` as JSON.
    pub fn new_from(path: impl Into<PathBuf>, contents: &str) -> Result<Self, NanoDBError> {
        let data = serde_json::from_str(&contents)?;
        let _path: PathBuf;
        if cfg!(test) {
            let tmp_dir = tempdir()?;
            _path = tmp_dir.path().join("my_file.json");
        } else {
            _path = path.into();
            std::fs::write(&_path, &contents)?;
        }
        Ok(Self {
            path: _path,
            data: Arc::new(RwLock::new(data)),
        })
    }

    pub fn doctest_new_from(
        _path: impl Into<PathBuf>,
        contents: &str,
    ) -> Result<Self, NanoDBError> {
        let data = serde_json::from_str(&contents)?;
        let _path: PathBuf;
        let tmp_dir = tempdir()?;
        _path = tmp_dir.path().join("my_file.json");
        Ok(Self {
            path: _path,
            data: Arc::new(RwLock::new(data)),
        })
    }

    /// Retrieves the value associated with a given key in the JSON data of the NanoDB instance.
    ///
    /// # Arguments
    ///
    /// * `key` - The key to retrieve the value for.
    ///
    /// # Returns
    ///
    /// * `Ok(Tree)` - A new Tree object that represents the value associated with `key`.
    /// * `Err(NanoDBError::RwLockReadError)` - If there was an error acquiring the read lock.
    /// * `Err(NanoDBError::KeyNotFound(key))` - If `key` does not exist in the JSON data.
    /// # Examples
    ///
    /// ```
    /// # use nanodb::nanodb::NanoDB;
    /// # use serde_json::json;
    /// # let db = NanoDB::doctest_new_from("/path/to/file.json", r#"{"key": "value"}"#).unwrap();
    /// // Data: {"key": "value"}
    /// assert_eq!(db.get("key").unwrap().inner(), json!("value"));
    /// ```
    pub fn get(&self, key: &str) -> Result<Tree, NanoDBError> {
        let data = self
            .data
            .read()
            .map_err(|e| NanoDBError::RwLockReadError(e.to_string()))?;
        let value = data
            .get(key)
            .ok_or_else(|| NanoDBError::KeyNotFound(key.to_string()))?;
        Ok(Tree::new(
            value.clone(),
            vec![PathStep::Key(key.to_string())],
        ))
    }

    /// Inserts a key-value pair into the JSON data of the NanoDB instance.
    ///
    /// # Arguments
    ///
    /// * `key` - The key to insert the value for.
    /// * `value` - The value to insert. This value must implement the `Serialize` trait.
    ///
    /// # Returns
    ///
    /// * `Ok(())` - If the operation was successful.
    /// * `Err(NanoDBError::RwLockReadError)` - If there was an error acquiring the write lock.
    /// * `Err(serde_json::Error)` - If there was an error serializing `value`.
    ///
    /// # Examples
    ///
    /// ```
    /// # use nanodb::nanodb::NanoDB;
    /// # use serde_json::json;
    /// # let mut db = NanoDB::doctest_new_from("/path/to/file.json", r#"{"key": "value"}"#).unwrap();
    /// db.insert("key", "value");
    /// db.insert("key2", [1,2,3]);
    /// ```
    pub fn insert<T: Serialize>(&mut self, key: &str, value: T) -> Result<(), NanoDBError> {
        let mut data = self
            .data
            .write()
            .map_err(|e| NanoDBError::RwLockReadError(e.to_string()))?;
        let value = serde_json::to_value(value)?;
        data.as_object_mut().unwrap().insert(key.to_string(), value);
        Ok(())
    }

    /// Merges a Tree into the JSON data of the NanoDB instance at a given path.
    ///
    /// # Arguments
    ///
    /// * `tree` - The Tree to merge into the JSON data.
    ///
    /// # Returns
    ///
    /// * `Ok(())` - If the operation was successful.
    /// * `Err(NanoDBError::RwLockWriteError)` - If there was an error acquiring the write lock.
    /// * `Err(NanoDBError::InvalidJSONPath)` - If the path does not exist in the JSON data or if a path step is not valid for the current value (e.g., using a key on an array or an index on an object).
    /// * `Err(NanoDBError::IndexOutOfBounds)` - If an index path step is out of bounds of the array.
    ///
    /// # Examples
    ///
    /// ```
    /// # use nanodb::nanodb::NanoDB;
    /// # use serde_json::json;
    /// # let mut db = NanoDB::doctest_new_from(
    /// #    "/path/to/file.json",
    /// #    r#"{"key": {"nested_key": "nested_value"}}"#,
    /// # )
    /// # .unwrap();
    /// // Data: {"key": {"nested_key": "nested_value"}}
    /// let mut tree = db.get("key").unwrap();
    /// tree.insert("nested_key_2", "nested_value_2").unwrap();
    /// db.merge(tree).unwrap();
    /// assert_eq!(
    ///     db.get("key").unwrap().get("nested_key_2").unwrap().inner(),
    ///     json!("nested_value_2")
    /// );
    /// ```
    pub fn merge(&mut self, tree: Tree) -> Result<(), NanoDBError> {
        let path = tree.path();
        let mut data = self._write_lock()?;

        let mut current = &mut *data;
        for p in path {
            match p {
                PathStep::Key(key) => {
                    if current.is_object() {
                        let obj = current.as_object_mut().unwrap();
                        // Check if the key exists, and if so, get a mutable reference to it.
                        // Otherwise, return an error.
                        match obj.get_mut(&key) {
                            Some(value) => current = value,
                            None => return Err(NanoDBError::InvalidJSONPath),
                        }
                    } else {
                        return Err(NanoDBError::InvalidJSONPath);
                    }
                }
                PathStep::Index(idx) => {
                    if current.is_array() {
                        let arr = current.as_array_mut().unwrap();
                        current = arr.get_mut(idx).ok_or(NanoDBError::IndexOutOfBounds(idx))?;
                    } else {
                        return Err(NanoDBError::InvalidJSONPath);
                    }
                }
            }
        }

        *current = tree.inner();
        Ok(())
    }

    /// Merges a Tree into the JSON data of the NanoDB instance and writes the data to the file.
    ///
    /// # Arguments
    ///
    /// * `tree` - The Tree to merge into the JSON data.
    ///
    /// # Returns
    ///
    /// * `Ok(())` - If the operation was successful.
    /// * `Err(NanoDBError::RwLockWriteError)` - If there was an error acquiring the write lock.
    /// * `Err(NanoDBError::InvalidJSONPath)` - If the path does not exist in the JSON data or if a path step is not valid for the current value (e.g., using a key on an array or an index on an object).
    /// * `Err(NanoDBError::IndexOutOfBounds)` - If an index path step is out of bounds of the array.
    /// * `Err(NanoDBError::FileWriteError)` - If there was an error writing the data to the file.
    pub fn merge_and_write(&mut self, tree: Tree) -> Result<(), NanoDBError> {
        self.merge(tree)?;
        self.write()
    }

    /// Writes the JSON data of the NanoDB instance to the file at its path.
    ///
    /// # Returns
    ///
    /// * `Ok(())` - If the operation was successful.
    /// * `Err(NanoDBError::RwLockWriteError)` - If there was an error acquiring the write lock.
    /// * `Err(serde_json::Error)` - If there was an error serializing the JSON data.
    /// * `Err(std::io::Error)` - If there was an error writing the data to the file.
    pub fn write(&mut self) -> Result<(), NanoDBError> {
        let data_guard = self
            .data
            .write()
            .map_err(|e| NanoDBError::RwLockWriteError(e.to_string()))?;
        let contents = serde_json::to_string_pretty(&*data_guard)?;
        std::fs::write(&self.path, contents)?;
        Ok(())
    }

    /// Write the current state of the JSON data to disk asynchronously
    pub async fn write_async(&self) -> Result<(), NanoDBError> {
        let data_guard = self
            .data
            .write()
            .map_err(|e| NanoDBError::RwLockWriteError(e.to_string()))?;
        let contents = serde_json::to_string_pretty(&*data_guard)?;
        tokio::fs::write(&self.path, contents).await?;
        Ok(())
    }

    fn _write_lock(&mut self) -> Result<RwLockWriteGuard<'_, Value>, NanoDBError> {
        self.data
            .write()
            .map_err(|e| NanoDBError::RwLockWriteError(e.to_string()))
    }

    fn _read_lock(&mut self) -> Result<RwLockReadGuard<'_, Value>, NanoDBError> {
        self.data
            .read()
            .map_err(|e| NanoDBError::RwLockReadError(e.to_string()))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::json;

    #[test]
    fn test_new_from() {
        let db = NanoDB::new_from("/path/to/file.json", r#"{"key": "value"}"#).unwrap();
        assert_eq!(db.get("key").unwrap().inner(), json!("value"));
    }

    #[test]
    fn test_insert() {
        let mut db = NanoDB::new_from("/path/to/file.json", r#"{}"#).unwrap();
        db.insert("new_key", "new_value").unwrap();
        assert_eq!(db.get("new_key").unwrap().inner(), json!("new_value"));
    }

    #[test]
    fn test_get() {
        let db = NanoDB::new_from("/path/to/file.json", r#"{"key": "value"}"#).unwrap();
        let result = db.get("key").unwrap();
        assert_eq!(result.inner(), json!("value"));
    }

    #[test]
    fn test_merge() {
        let mut db = NanoDB::new_from(
            "/path/to/file.json",
            r#"{"key": {"nested_key": "nested_value"}}"#,
        )
        .unwrap();
        let mut tree = db.get("key").unwrap();
        tree.insert("nested_key_2", "nested_value_2").unwrap();
        db.merge(tree).unwrap();
        assert_eq!(
            db.get("key").unwrap().get("nested_key_2").unwrap().inner(),
            json!("nested_value_2")
        );
    }
}
