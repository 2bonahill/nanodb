use anyhow::anyhow;
use serde::{Deserialize, Serialize};

use crate::error::NanoDBError;

#[derive(Debug, Clone)]
pub struct ValueTree {
    pub inner: serde_json::Value,
    pub path: Vec<PathStep>,
}

#[derive(Debug, Clone)]
pub enum PathStep {
    Key(String),
    Index(usize),
}

#[derive(Debug, Clone)]
pub enum ValueTreeType {
    Null,
    Bool,
    Number,
    String,
    Array,
    Object,
}

impl ValueTree {
    // This method allows chaining by returning another ValueWrapper.
    pub fn get(&self, key: &str) -> Result<ValueTree, NanoDBError> {
        match &self.inner {
            serde_json::Value::Object(map) => {
                let value = map
                    .get(key)
                    .ok_or_else(|| anyhow!("Key not found: {}", key))?;
                let mut new_path: Vec<PathStep> = self.path.clone();
                new_path.push(PathStep::Key(key.to_string()));
                Ok(ValueTree {
                    inner: value.clone(),
                    path: new_path,
                })
            }
            _ => Err(NanoDBError::NotAnObject),
        }
    }

    pub fn at(&self, index: usize) -> Result<ValueTree, NanoDBError> {
        match &self.inner {
            serde_json::Value::Array(arr) => {
                let value = arr
                    .get(index)
                    .ok_or_else(|| anyhow!("Index out of bounds: {}", index))?;
                let mut new_path: Vec<PathStep> = self.path.clone();
                new_path.push(PathStep::Index(index));
                Ok(ValueTree {
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

    pub fn tree_type(&self) -> ValueTreeType {
        match &self.inner {
            serde_json::Value::Null => ValueTreeType::Null,
            serde_json::Value::Bool(_) => ValueTreeType::Bool,
            serde_json::Value::Number(_) => ValueTreeType::Number,
            serde_json::Value::String(_) => ValueTreeType::String,
            serde_json::Value::Array(_) => ValueTreeType::Array,
            serde_json::Value::Object(_) => ValueTreeType::Object,
        }
    }

    /// Push a value to an array
    /// If self is not of type array, an error is returned
    /// If the value cannot be serialized, an error is returned
    /// If the value is successfully pushed, Ok is returned
    pub fn push<T: Serialize>(&mut self, value: T) -> Result<ValueTree, NanoDBError> {
        let value = serde_json::to_value(value)?;

        if let Some(v) = self.inner.as_array_mut() {
            v.push(value);
        } else {
            return Err(NanoDBError::NotAnArray);
        }

        Ok(self.clone())
    }

    /// Apply a closure to each element of an array
    /// If self is not of type array, an error is returned
    pub fn for_each<F>(&mut self, f: F) -> Result<ValueTree, NanoDBError>
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
}
