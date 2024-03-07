use serde::Serialize;
use serde_json::Value;
use std::{path::PathBuf, sync::Arc};
use tempfile::tempdir;
use tokio::sync::{RwLock, RwLockReadGuard, RwLockWriteGuard};

use crate::{
    error::NanoDBError,
    trees::{tree::Tree, tree_read_guarded::ReadGuardedTree, tree_write_guarded::WriteGuardedTree},
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
        let data = serde_json::from_str(contents)?;
        let _path: PathBuf;
        if cfg!(test) {
            let tmp_dir = tempdir()?;
            _path = tmp_dir.path().join("my_file.json");
        } else {
            _path = path.into();
            std::fs::write(&_path, contents)?;
        }
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
    pub async fn data(&self) -> Tree {
        let data = self._read_lock().await;
        Tree::new(data.clone(), vec![])
    }

    /// Executes an atomic query to the db, ensuring that the query either fully completes
    /// or is entirely rolled back in case of an error, maintaining the integrity of the database.
    /// This function is designed to handle operations that must be executed as a single,
    /// indivisible unit to ensure data consistency and reliability, such as transactions
    /// involving multiple steps.
    ///
    /// Returns a read-guarded tree.
    ///
    /// # Returns
    ///
    /// * `Ok(ReadGuardedTree)` - A new ReadGuardedTree instance with the read lock and the JSON data.
    /// * `Err(NanoDBError::RwLockReadError)` - If there was an error acquiring the read lock.
    pub async fn read(&self) -> ReadGuardedTree<'_> {
        let read_guard = self._read_lock().await;
        let value: Value = read_guard.clone();
        ReadGuardedTree::new(read_guard, value)
    }

    /// Asynchronously returns a write-guarded tree.
    ///
    /// # Returns
    ///
    /// * `Ok(GuardedTree)` - A new GuardedTree instance with the write lock and the JSON data.
    /// * `Err(NanoDBError::RwLockWriteError)` - If there was an error acquiring the write lock.
    pub async fn update(&self) -> WriteGuardedTree<'_> {
        let write_guard = self._write_lock().await;
        let value: Value = write_guard.clone();
        WriteGuardedTree::new(write_guard, value)
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
    pub async fn insert<T: Serialize>(&mut self, key: &str, value: T) -> Result<(), NanoDBError> {
        let tree_guard = self._write_lock().await;
        let tree_value = tree_guard.clone();
        let mut tree = WriteGuardedTree::new(tree_guard, tree_value);
        tree.insert(key, value)?;
        Ok(())
    }

    /// Merges a Tree (other) into the JSON data of the NanoDB instance
    /// It does so by respecting the path of the other Tree instance.
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
    pub async fn merge_from(&mut self, other: Tree) -> Result<(), NanoDBError> {
        let mut current = self._write_lock().await;

        // wrap data into a tree to use the merge from method
        let mut current_tree = Tree::new(current.clone(), vec![]);
        current_tree.merge_from(other)?;

        // update the current write guarded value
        *current = current_tree.inner();

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
    pub async fn merge_and_write(&mut self, tree: Tree) -> Result<(), NanoDBError> {
        self.merge_from(tree).await?;
        self.write().await?;
        Ok(())
    }

    /// Writes the JSON data of the NanoDB instance to the file at its path.
    ///
    /// # Returns
    ///
    /// * `Ok(())` - If the operation was successful.
    /// * `Err(NanoDBError::RwLockWriteError)` - If there was an error acquiring the write lock.
    /// * `Err(serde_json::Error)` - If there was an error serializing the JSON data.
    /// * `Err(std::io::Error)` - If there was an error writing the data to the file.
    pub async fn write(&mut self) -> Result<(), NanoDBError> {
        let path = self.path.clone();
        let data_guard = self._write_lock().await;
        let contents = serde_json::to_string_pretty(&*data_guard)?;
        tokio::fs::write(path, contents).await?;
        Ok(())
    }

    async fn _write_lock(&self) -> RwLockWriteGuard<'_, Value> {
        self.data.write().await
    }

    async fn _read_lock(&self) -> RwLockReadGuard<'_, Value> {
        self.data.read().await
    }
}

impl Clone for NanoDB {
    fn clone(&self) -> Self {
        Self {
            path: self.path.clone(),
            data: self.data.clone(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::json;

    #[tokio::test]
    async fn test_new_from() {
        let db = NanoDB::new_from("/path/to/file.json", r#"{"key": "value"}"#).unwrap();
        assert_eq!(db.data().await.get("key").unwrap().inner(), json!("value"));
    }

    #[tokio::test]
    async fn test_insert() {
        let mut db = NanoDB::new_from("/path/to/file.json", r#"{}"#).unwrap();
        db.insert("new_key", "new_value").await.unwrap();
        assert_eq!(
            db.data().await.get("new_key").unwrap().inner(),
            json!("new_value")
        );
    }

    #[tokio::test]
    async fn test_get() {
        let db = NanoDB::new_from("/path/to/file.json", r#"{"key": "value"}"#).unwrap();
        let result = db.data().await.get("key").unwrap();
        assert_eq!(result.inner(), json!("value"));
    }

    #[tokio::test]
    async fn test_merge() {
        let mut db = NanoDB::new_from(
            "/path/to/file.json",
            r#"{"key": {"nested_key": "nested_value"}}"#,
        )
        .unwrap();
        let mut tree = db.data().await.get("key").unwrap();
        tree.insert("nested_key_2", "nested_value_2").unwrap();
        db.merge_from(tree).await.unwrap();
        assert_eq!(
            db.data()
                .await
                .get("key")
                .unwrap()
                .get("nested_key_2")
                .unwrap()
                .inner(),
            json!("nested_value_2")
        );
    }
}
