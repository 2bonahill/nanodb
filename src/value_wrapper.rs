use anyhow::anyhow;
use serde::Deserialize;

use crate::error::NanoDBError;

#[derive(Debug)]
pub struct ValueWrapper {
    pub inner: serde_json::Value,
    pub path: Vec<PathElement>,
}

#[derive(Debug, Clone)]
pub enum PathElement {
    Key(String),
    Index(usize),
}

impl ValueWrapper {
    // This method allows chaining by returning another ValueWrapper.
    pub fn get(&self, key: &str) -> Result<ValueWrapper, NanoDBError> {
        match &self.inner {
            serde_json::Value::Object(map) => {
                let value = map
                    .get(key)
                    .ok_or_else(|| anyhow!("Key not found: {}", key))?;
                let mut new_path: Vec<PathElement> = self.path.clone();
                new_path.push(PathElement::Key(key.to_string()));
                Ok(ValueWrapper {
                    inner: value.clone(),
                    path: new_path,
                })
            }
            _ => Err(NanoDBError::NotAnObject),
        }
    }

    pub fn at(&self, index: usize) -> Result<ValueWrapper, NanoDBError> {
        match &self.inner {
            serde_json::Value::Array(arr) => {
                let value = arr
                    .get(index)
                    .ok_or_else(|| anyhow!("Index out of bounds: {}", index))?;
                let mut new_path: Vec<PathElement> = self.path.clone();
                new_path.push(PathElement::Index(index));
                Ok(ValueWrapper {
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

    pub fn path(&self) -> Vec<PathElement> {
        self.path.clone()
    }
}
