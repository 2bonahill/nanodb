use serde::Serialize;
use serde_json::Value;
use tokio::sync::RwLockWriteGuard;

use crate::error::NanoDBError;

use super::tree::{PathStep, Tree};

// Define the WriteGuardedTree struct
#[derive(Debug)]
pub struct WriteGuardedTree<'a> {
    _guard: RwLockWriteGuard<'a, Value>,
    inner: Tree,
}

impl<'a> WriteGuardedTree<'a> {
    // Constructor for a WriteGuardedTree
    pub(crate) fn new(guard: RwLockWriteGuard<'a, Value>, value: Value) -> Self {
        let tree = Tree::new(value, vec![]);
        WriteGuardedTree {
            _guard: guard,
            inner: tree,
        }
    }

    /// Retrieves the value associated with a given key in the JSON data of the TreeWriteGuarded instance.
    ///
    /// # Arguments
    ///
    /// * `key` - The key to retrieve the value for.
    ///
    /// # Returns
    ///
    /// * `Ok(&mut Self)` - The TreeWriteGuarded instance itself after the retrieval. This allows for method chaining.
    /// * `Err(NanoDBError::InvalidJSONPath)` - If the path to the key in the JSON data is invalid.
    pub fn get(&mut self, key: &str) -> Result<&mut Self, NanoDBError> {
        self.inner = self.inner.clone().get(key)?;
        Ok(self)
    }

    /// Retrieves the value at a given index in the JSON array of the TreeWriteGuarded instance.
    ///
    /// # Arguments
    ///
    /// * `index` - The index to retrieve the value from.
    ///
    /// # Returns
    ///
    /// * `Ok(&mut Self)` - The TreeWriteGuarded instance itself after the retrieval. This allows for method chaining.
    /// * `Err(NanoDBError::InvalidJSONPath)` - If the path to the index in the JSON data is invalid.
    /// * `Err(NanoDBError::IndexOutOfBounds)` - If the index is out of bounds.
    pub fn at(&mut self, index: usize) -> Result<&mut Self, NanoDBError> {
        self.inner = self.inner.clone().at(index)?;
        Ok(self)
    }

    /// Inserts a key-value pair into the inner JSON object of the TreeWriteGuarded instance.
    ///
    /// # Arguments
    ///
    /// * `key` - The key to insert the value for.
    /// * `value` - The value to insert. This value must implement the `Serialize` trait.
    ///
    /// # Returns
    ///
    /// * `Ok(&mut Self)` - The TreeWriteGuarded instance itself after the insertion. This allows for method chaining.
    /// * `Err(NanoDBError::InvalidJSONPath)` - If the path to the key in the JSON data is invalid.
    /// * `Err(NanoDBError::IndexOutOfBounds)` - If an array index in the path is out of bounds.
    pub fn insert<T: Serialize>(&mut self, key: &str, value: T) -> Result<&mut Self, NanoDBError> {
        self.inner = self.inner.insert(key, value)?;

        self.merge_from(self.inner.clone())?;

        Ok(self)
    }

    /// Converts the inner JSON object of the TreeWriteGuarded instance into a specified type.
    ///
    /// # Type Parameters
    ///
    /// * `T` - The type to convert the JSON object into. This type must implement the `Deserialize` trait.
    ///
    /// # Returns
    ///
    /// * `Ok(T)` - The JSON object converted into the specified type.
    /// * `Err(serde_json::Error)` - If there was an error during the conversion.
    pub fn into<T: for<'de> serde::Deserialize<'de>>(&mut self) -> Result<T, serde_json::Error> {
        serde_json::from_value(self.inner.inner())
    }

    /// Merges the JSON data from another Tree instance into this guarded instance.
    ///
    /// # Arguments
    ///
    /// * `other` - The other Tree instance to merge from.
    fn merge_from(&mut self, other: Tree) -> Result<&mut Self, NanoDBError> {
        let path = self.inner.path();
        let mut current = &mut *self._guard;

        for p in path {
            match p {
                PathStep::Key(key) => {
                    if current.is_object() {
                        let obj = current.as_object_mut().unwrap();
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

        *current = other.inner();

        Ok(self)
    }
}
