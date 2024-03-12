use serde::Serialize;
use serde_json::Value;
use tokio::sync::RwLockWriteGuard;

use crate::error::NanoDBError;

use super::tree::Tree;

/// A struct representing a write-guarded tree.
///
/// This struct contains a write lock guard and a tree. The write lock guard ensures that only one thread can modify the tree at a time.
///
/// # Fields
///
/// * `_guard` - The write lock guard. This is not directly used, but its existence ensures that the tree cannot be modified by other threads.
/// * `inner` - The tree that is being guarded.
#[derive(Debug)]
pub struct WriteGuardedTree<'a> {
    _guard: RwLockWriteGuard<'a, Value>,
    tree: Tree,
}

impl<'a> WriteGuardedTree<'a> {
    /// Creates a new WriteGuardedTree instance.
    ///
    /// # Arguments
    ///
    /// * `guard` - The write lock guard. This is not directly used, but its existence ensures that the tree cannot be modified by other threads.
    /// * `value` - The initial JSON value of the tree.
    ///
    /// # Returns
    ///
    /// * `WriteGuardedTree` - The new WriteGuardedTree instance.
    pub(crate) fn new(guard: RwLockWriteGuard<'a, Value>, value: Value) -> Self {
        let tree = Tree::new(value, vec![]);
        WriteGuardedTree {
            _guard: guard,
            tree,
        }
    }

    /// Releases the write lock guard of the TreeWriteGuarded instance.
    ///
    /// This function consumes the TreeWriteGuarded instance and drops it, which releases the write lock guard.
    pub fn release_lock(self) {
        drop(self);
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
        self.tree = self.tree.get(key)?;
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
        self.tree = self.tree.at(index)?;
        Ok(self)
    }

    /// Inserts a key-value pair into the inner JSON object of the TreeWriteGuarded instance,
    /// at the current path of the tree.
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
        self.tree = self.tree.clone().insert(key, value)?;
        self.merge()?;
        Ok(self)
    }

    /// Removes a key-value pair from the inner JSON object of the TreeWriteGuarded instance and then merges the result into the current JSON value of the write lock guard.
    ///
    /// # Arguments
    ///
    /// * `key` - The key to remove the value for.
    ///
    /// # Returns
    ///
    /// * `Ok(&mut Self)` - The TreeWriteGuarded instance itself after the removal and merge. This allows for method chaining.
    /// * `Err(NanoDBError)` - If there was an error during the removal or the merge.
    pub fn remove(&mut self, key: &str) -> Result<&mut Self, NanoDBError> {
        self.tree = self.tree.clone().remove(key)?;
        self.merge()?;
        Ok(self)
    }

    /// Pushes a value to the tree if it's currently pointing to an array.
    ///
    /// # Arguments
    ///
    /// * `value` - A value of type T that implements the Serialize trait. This value will be serialized to JSON and pushed to the array.
    ///
    /// # Returns
    ///
    /// * `Ok(Tree)` - A new Tree object that represents the current state of the tree after the value has been pushed.
    /// * `Err(NanoDBError::NotAnArray)` - If the inner value of the tree is not an array.
    pub fn push<T: Serialize>(&mut self, value: T) -> Result<&mut Self, NanoDBError> {
        self.tree = self.tree.clone().push(value)?;
        self.merge()?;
        Ok(self)
    }

    /// Applies a function to each element of the inner array of the tree.
    ///
    /// # Arguments
    ///
    /// * `f` - A mutable function that takes a mutable reference to a `serde_json::Value` and returns `()`.
    ///
    /// # Returns
    ///
    /// * `Ok(Tree)` - A new Tree object that represents the current state of the tree after the function has been applied to each element.
    /// * `Err(NanoDBError::NotAnArray)` - If the inner value of the tree is not an array.
    pub fn for_each<F>(&mut self, f: F) -> Result<&mut Self, NanoDBError>
    where
        F: FnMut(&mut serde_json::Value),
    {
        self.tree = self.tree.clone().for_each(f)?;
        self.merge()?;
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
        serde_json::from_value(self.tree.inner())
    }

    /// Merges the inner Tree (self.tree) instance into the the write lock guard.
    ///
    /// # Returns
    ///
    /// * `Ok(&mut Self)` - The TreeWriteGuarded instance itself after the merge. This allows for method chaining.
    /// * `Err(NanoDBError)` - If there was an error during the merge.
    pub fn merge(&mut self) -> Result<&mut Self, NanoDBError> {
        let current = &mut *self._guard;

        // Wrap it in a Tree so we can use the standard tree method to merge
        let mut current_wrapped = Tree::new(current.clone(), vec![]);
        current_wrapped.merge_from(self.tree.clone())?;

        // Unwrap the value and assign it to the guard
        *current = current_wrapped.inner();

        Ok(self)
    }

    /// get snapshot of the tree
    pub fn tree(&self) -> &Tree {
        &self.tree
    }
}

#[cfg(test)]
mod tests {

    use crate::{
        nanodb::NanoDB,
        trees::tree::{PathStep, Tree},
    };
    use serde_json::{json, Value};

    fn value() -> Value {
        serde_json::from_str(
            r#"{
			"key1": "value1",
			"key2": {
				"inner_key1": "inner_value1",
				"inner_key2": "inner_value2"
			},
			"key3": [1, 2, 3]
		}"#,
        )
        .unwrap()
    }

    fn value_str() -> String {
        r#"{
			"key1": "value1",
			"key2": {
				"inner_key1": "inner_value1",
				"inner_key2": "inner_value2"
			},
			"key3": [1, 2, 3]
		}"#
        .to_string()
    }

    #[tokio::test]
    async fn test_write_guarded_new() {
        let value = value();
        let rwlock = tokio::sync::RwLock::new(value.clone());
        let guard = rwlock.write().await;
        let tree = Tree::new(value.clone(), vec![]);
        let write_guarded = super::WriteGuardedTree::new(guard, value.clone());
        assert_eq!(write_guarded.tree.inner(), tree.inner());
    }

    #[tokio::test]
    async fn test_write_guarded_get() {
        let db = NanoDB::new_from("/path/to/file.json", &value_str()).unwrap();
        let mut write_guarded = db.update().await;

        write_guarded.get("key2").unwrap();
        let tree = Tree::new(
            json!({
                "inner_key1": "inner_value1",
                "inner_key2": "inner_value2"
            }),
            vec![PathStep::Key("key2".to_string())],
        );
        assert_eq!(write_guarded.tree.inner(), tree.inner());
    }

    #[tokio::test]
    async fn test_write_guarded_at() {
        let db = NanoDB::new_from("/path/to/file.json", &value_str()).unwrap();
        let mut write_guarded = db.update().await;
        write_guarded.get("key3").unwrap().at(1).unwrap();
        let tree = Tree::new(
            json!(2),
            vec![PathStep::Key("key3".to_string()), PathStep::Index(1)],
        );
        assert_eq!(write_guarded.tree.inner(), tree.inner());
    }

    #[tokio::test]
    async fn test_write_guarded_insert() {
        let db = NanoDB::new_from("/path/to/file.json", &value_str()).unwrap();
        let mut write_guarded = db.update().await;
        write_guarded
            .get("key2")
            .unwrap()
            .insert("inner_key3", "inner_value3")
            .unwrap();
        let tree = Tree::new(
            json!({
                "inner_key1": "inner_value1",
                "inner_key2": "inner_value2",
                "inner_key3": "inner_value3"
            }),
            vec![PathStep::Key("key2".to_string())],
        );
        assert_eq!(write_guarded.tree.inner(), tree.inner());
    }

    #[tokio::test]
    async fn test_write_guarded_remove() {
        let db = NanoDB::new_from("/path/to/file.json", &value_str()).unwrap();
        let mut write_guarded = db.update().await;
        write_guarded
            .get("key2")
            .unwrap()
            .remove("inner_key1")
            .unwrap();
        let tree = Tree::new(
            json!({
                "inner_key2": "inner_value2"
            }),
            vec![PathStep::Key("key2".to_string())],
        );
        assert_eq!(write_guarded.tree.inner(), tree.inner());

        write_guarded.release_lock();
    }
}
