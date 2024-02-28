use anyhow::anyhow;
use serde::{Deserialize, Serialize};

use crate::error::NanoDBError;

#[derive(Debug, Clone)]
pub struct Tree {
    pub inner: serde_json::Value,
    pub path: Vec<PathStep>,
}

#[derive(Debug, Clone)]
pub enum PathStep {
    Key(String),
    Index(usize),
}

#[derive(Debug, Clone)]
pub enum TreeType {
    Null,
    Bool,
    Number,
    String,
    Array,
    Object,
}

impl Tree {
    pub fn get(&self, key: &str) -> Result<Tree, NanoDBError> {
        match &self.inner {
            serde_json::Value::Object(map) => {
                let value = map
                    .get(key)
                    .ok_or_else(|| anyhow!("Key not found: {}", key))?;
                let mut new_path: Vec<PathStep> = self.path.clone();
                new_path.push(PathStep::Key(key.to_string()));
                Ok(Tree {
                    inner: value.clone(),
                    path: new_path,
                })
            }
            _ => Err(NanoDBError::NotAnObject),
        }
    }

    pub fn at(&self, index: usize) -> Result<Tree, NanoDBError> {
        match &self.inner {
            serde_json::Value::Array(arr) => {
                let value = arr
                    .get(index)
                    .ok_or_else(|| anyhow!("Index out of bounds: {}", index))?;
                let mut new_path: Vec<PathStep> = self.path.clone();
                new_path.push(PathStep::Index(index));
                Ok(Tree {
                    inner: value.clone(),
                    path: new_path,
                })
            }
            _ => Err(NanoDBError::NotAnArray),
        }
    }

    pub fn into<T: for<'de> Deserialize<'de>>(self) -> Result<T, NanoDBError> {
        serde_json::from_value(self.inner).map_err(Into::into)
    }

    pub fn path(&self) -> Vec<PathStep> {
        self.path.clone()
    }

    pub fn tree_type(&self) -> TreeType {
        match &self.inner {
            serde_json::Value::Null => TreeType::Null,
            serde_json::Value::Bool(_) => TreeType::Bool,
            serde_json::Value::Number(_) => TreeType::Number,
            serde_json::Value::String(_) => TreeType::String,
            serde_json::Value::Array(_) => TreeType::Array,
            serde_json::Value::Object(_) => TreeType::Object,
        }
    }

    /// Pushes a value to the tree if it's an array.
    ///
    /// # Arguments
    ///
    /// * `value` - A value of type T that implements the Serialize trait. This value will be serialized to JSON and pushed to the array.
    ///
    /// # Returns
    ///
    /// * `Ok(Tree)` - A new Tree object that represents the current state of the tree after the value has been pushed.
    /// * `Err(NanoDBError::NotAnArray)` - If the inner value of the tree is not an array.
    ///
    /// # Examples
    ///
    /// ```
    /// let mut tree = Tree::new(serde_json::json!([1, 2, 3]));
    /// let result = tree.push(4);
    /// assert_eq!(result.unwrap().inner, serde_json::json!([1, 2, 3, 4]));
    /// ```
    pub fn array_push<T: Serialize>(&mut self, value: T) -> Result<Tree, NanoDBError> {
        let value = serde_json::to_value(value)?;

        if let Some(v) = self.inner.as_array_mut() {
            v.push(value);
        } else {
            return Err(NanoDBError::NotAnArray);
        }

        Ok(self.clone())
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
    ///
    /// # Examples
    ///
    /// ```
    /// let mut tree = Tree::new(serde_json::json!([1, 2, 3]));
    /// let result = tree.for_each(|x| *x = serde_json::json!(*x.as_i64().unwrap() + 1));
    /// assert_eq!(result.unwrap().inner, serde_json::json!([2, 3, 4]));
    /// ```
    pub fn array_for_each<F>(&mut self, f: F) -> Result<Tree, NanoDBError>
    where
        F: FnMut(&mut serde_json::Value),
    {
        if let Some(v) = self.inner.as_array_mut() {
            v.iter_mut().for_each(f);
        } else {
            return Err(NanoDBError::NotAnArray);
        }

        Ok(self.clone())
    }

    /// Returns the length of the inner array of the tree.
    ///
    /// # Returns
    ///
    /// * `Ok(usize)` - The length of the array if the inner value of the tree is an array.
    /// * `Err(NanoDBError::NotAnArray)` - If the inner value of the tree is not an array.
    ///
    /// # Examples
    ///
    /// ```
    /// let tree = Tree::new(serde_json::json!([1, 2, 3]));
    /// let result = tree.array_len();
    /// assert_eq!(result.unwrap(), 3);
    /// ```
    pub fn array_len(&self) -> Result<usize, NanoDBError> {
        match &self.inner {
            serde_json::Value::Array(arr) => Ok(arr.len()),
            _ => Err(NanoDBError::NotAnArray),
        }
    }
}
