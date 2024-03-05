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

    // Implement methods specific to WriteGuardedTree here
    pub fn get(&mut self, key: &str) -> Result<&mut Self, NanoDBError> {
        self.inner = self.inner.clone().get(key)?;
        Ok(self)
    }

    pub fn at(&mut self, index: usize) -> Result<&mut Self, NanoDBError> {
        self.inner = self.inner.clone().at(index)?;
        Ok(self)
    }

    pub fn insert<T: Serialize>(&mut self, key: &str, value: T) -> Result<&mut Self, NanoDBError> {
        dbg!("hi from insert");
        dbg!(&self.inner);
        self.inner = self.inner.insert(key, value)?;
        dbg!(&self.inner);

        // *self._guard = self.inner.inner();
        self.merge_from(self.inner.clone())?;

        dbg!("hi back from merge_from");
        dbg!(&self);

        Ok(self)
    }

    pub fn into<T: for<'de> serde::Deserialize<'de>>(&mut self) -> Result<T, serde_json::Error> {
        serde_json::from_value(self.inner.inner())
    }

    /// Merges the JSON data from another Tree instance into this Tree instance.
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
