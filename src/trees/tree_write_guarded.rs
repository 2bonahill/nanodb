use serde::Serialize;
use serde_json::Value;
use tokio::sync::RwLockWriteGuard;

use crate::error::NanoDBError;

use super::tree::Tree;

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
        self.inner = self.inner.insert(key, value)?;
        *self._guard = self.inner.inner();
        Ok(self)
    }

    pub fn into<T: for<'de> serde::Deserialize<'de>>(&mut self) -> Result<T, serde_json::Error> {
        serde_json::from_value(self.inner.inner())
    }
}
