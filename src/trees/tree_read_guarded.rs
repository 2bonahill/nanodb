use serde_json::Value;
use tokio::sync::RwLockReadGuard;

use crate::error::NanoDBError;

use super::tree::Tree;

// Define the ReadGuardedTree struct
#[derive(Debug)]
pub struct ReadGuardedTree<'a> {
    _guard: RwLockReadGuard<'a, Value>,
    inner: Tree,
}

impl<'a> ReadGuardedTree<'a> {
    // Constructor for a ReadGuardedTree
    pub(crate) fn new(guard: RwLockReadGuard<'a, Value>, value: Value) -> Self {
        let tree = Tree::new(value, vec![]);
        ReadGuardedTree {
            _guard: guard,
            inner: tree,
        }
    }

    // Implement methods specific to ReadGuardedTree here
    pub fn get(&mut self, key: &str) -> Result<&mut Self, NanoDBError> {
        self.inner = self.inner.clone().get(key)?;
        Ok(self)
    }

    pub fn at(&mut self, index: usize) -> Result<&mut Self, NanoDBError> {
        self.inner = self.inner.clone().at(index)?;
        Ok(self)
    }

    pub fn into<T: for<'de> serde::Deserialize<'de>>(&mut self) -> Result<T, serde_json::Error> {
        serde_json::from_value(self.inner.inner())
    }
}
