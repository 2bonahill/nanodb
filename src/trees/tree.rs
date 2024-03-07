use serde::{Deserialize, Serialize};

use crate::error::NanoDBError;

#[derive(Debug, Clone)]
pub struct Tree {
    inner: serde_json::Value,
    path: Vec<PathStep>,
}

#[derive(Debug, Clone)]
pub enum PathStep {
    Key(String),
    Index(usize),
}

impl std::fmt::Display for PathStep {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            PathStep::Key(key) => write!(f, "{}", key),
            PathStep::Index(index) => write!(f, "[{}]", index),
        }
    }
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

// impl std::fmt::Display for Tree
impl std::fmt::Display for Tree {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Path: {} \nValue: {}", self.path_string(), self.inner)
    }
}

impl Tree {
    /// Creates a new Tree instance with the given value and path.
    ///
    /// # Arguments
    ///
    /// * `value` - The value to initialize the Tree with.
    /// * `path` - The path to the value in the original JSON object.
    ///
    /// # Returns
    ///
    /// * `Tree` - A new Tree instance with `value` as its inner value and `path` as its path.
    pub(crate) fn new(value: serde_json::Value, path: Vec<PathStep>) -> Self {
        Tree { inner: value, path }
    }

    /// Retrieves the value associated with a given key in the inner JSON object of the tree.
    ///
    /// # Arguments
    ///
    /// * `key` - The key to retrieve the value for.
    ///
    /// # Returns
    ///
    /// * `Ok(Tree)` - A new Tree object that represents the value associated with `key`.
    /// * `Err(NanoDBError::NotAnObject)` - If the inner value of the tree is not an object.
    /// * `Err(NanoDBError::KeyNotFound(key))` - If `key` does not exist in the JSON object.
    pub fn get(&self, key: &str) -> Result<Tree, NanoDBError> {
        match &self.inner {
            serde_json::Value::Object(map) => {
                let value = map
                    .get(key)
                    .ok_or_else(|| NanoDBError::KeyNotFound(key.to_string()))?;
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

    /// Retrieves the value at a given index in the inner JSON array of the tree.
    ///
    /// # Arguments
    ///
    /// * `index` - The index to retrieve the value from.
    ///
    /// # Returns
    ///
    /// * `Ok(Tree)` - A new Tree object that represents the value at `index`.
    /// * `Err(NanoDBError::NotAnArray)` - If the inner value of the tree is not an array.
    /// * `Err(NanoDBError::IndexOutOfBounds(index))` - If `index` is out of bounds of the array.
    pub fn at(&self, index: usize) -> Result<Tree, NanoDBError> {
        match &self.inner {
            serde_json::Value::Array(arr) => {
                let value = arr
                    .get(index)
                    .ok_or_else(|| NanoDBError::IndexOutOfBounds(index))?;
                let mut new_path: Vec<PathStep> = self.path.clone();
                new_path.push(PathStep::Index(index));
                Ok(Tree {
                    inner: value.clone(),
                    path: new_path,
                })
            }
            _ => Err(NanoDBError::NotAnArray(self.path_string())),
        }
    }

    /// Returns a clone of the inner JSON value of the Tree instance.
    ///
    /// # Returns
    ///
    /// * `serde_json::Value` - A clone of the inner JSON value.
    pub fn inner(&self) -> serde_json::Value {
        self.inner.clone()
    }

    /// Returns a clone of the path of the Tree instance.
    ///
    /// # Returns
    ///
    /// * `Vec<PathStep>` - A clone of the path.
    pub(crate) fn path(&self) -> Vec<PathStep> {
        self.path.clone()
    }

    /// Returns the path of the Tree instance as a dot-separated string.
    ///
    /// # Returns
    ///
    /// * `String` - The path as a dot-separated string.
    pub fn path_string(&self) -> String {
        self.path
            .iter()
            .map(|p| p.to_string())
            .collect::<Vec<String>>()
            .join(".")
    }

    /// Converts the inner JSON value of the Tree instance into a specified type.
    ///
    /// # Type Parameters
    ///
    /// * `T` - The type to convert the JSON value into. This type must implement the `Deserialize` trait.
    ///
    /// # Returns
    ///
    /// * `Ok(T)` - The JSON value converted into the specified type.
    /// * `Err(NanoDBError)` - If there was an error during the conversion.
    pub fn into<T: for<'de> Deserialize<'de>>(self) -> Result<T, NanoDBError> {
        serde_json::from_value(self.inner).map_err(Into::into)
    }

    /// Returns the type of the inner value of the tree.
    ///
    /// # Returns
    ///
    /// * `TreeType` - The type of the inner value of the tree. This can be one of `Null`, `Bool`, `Number`, `String`, `Array`, or `Object`.
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

    /// Inserts a key-value pair into the inner JSON object of the Tree instance.
    ///
    /// # Arguments
    ///
    /// * `key` - The key to insert the value for.
    /// * `value` - The value to insert. This value must implement the `Serialize` trait.
    ///
    /// # Returns
    ///
    /// * `Ok(Tree)` - The Tree instance itself after the insertion. This allows for method chaining.
    /// * `Err(NanoDBError::SerializationError)` - If there was an error serializing `value`.
    pub fn insert<T: Serialize>(&mut self, key: &str, value: T) -> Result<Tree, NanoDBError> {
        let value = serde_json::to_value(value)?;
        self.inner
            .as_object_mut()
            .unwrap()
            .insert(key.to_string(), value);
        Ok(self.clone())
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
    pub fn merge_from(&mut self, other: Tree) -> Result<&mut Self, NanoDBError> {
        let path = other.path();
        let mut current = &mut self.inner;

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
    pub fn push<T: Serialize>(&mut self, value: T) -> Result<Tree, NanoDBError> {
        let value = serde_json::to_value(value)?;

        if let Some(v) = self.inner.as_array_mut() {
            v.push(value);
        } else {
            return Err(NanoDBError::NotAnArray(self.path_string()));
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
    pub fn for_each<F>(&mut self, f: F) -> Result<Tree, NanoDBError>
    where
        F: FnMut(&mut serde_json::Value),
    {
        if let Some(v) = self.inner.as_array_mut() {
            v.iter_mut().for_each(f);
        } else {
            return Err(NanoDBError::NotAnArray(self.path_string()));
        }

        Ok(self.clone())
    }

    /// Returns the length of the inner array of the tree.
    ///
    /// # Returns
    ///
    /// * `Ok(usize)` - The length of the array if the inner value of the tree is an array.
    /// * `Err(NanoDBError::NotAnArray)` - If the inner value of the tree is not an array.
    pub fn len(&self) -> Result<usize, NanoDBError> {
        match &self.inner {
            serde_json::Value::Array(arr) => Ok(arr.len()),
            serde_json::Value::Object(obj) => Ok(obj.len()),
            _ => Err(NanoDBError::LenNotDefined(self.path_string())),
        }
    }

    /// Checks if the inner JSON array of the Tree instance is empty.
    ///
    /// # Returns
    ///
    /// * `true` - If the inner JSON value empty.
    /// * `false` - If the inner JSON value is not empty.
    pub fn is_empty(&self) -> bool {
        match &self.inner {
            serde_json::Value::Array(arr) => arr.is_empty(),
            serde_json::Value::Object(obj) => obj.is_empty(),
            _ => false,
        }
    }
}

#[cfg(test)]
mod tests {

    use crate::trees::tree::Tree;
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

    #[tokio::test]
    async fn test_tree_get() {
        let tree1 = Tree::new(value(), vec![]).get("key1").unwrap();
        assert_eq!(tree1.inner(), json!("value1"));

        let tree2 = Tree::new(value(), vec![]).get("key2").unwrap();
        assert_eq!(
            tree2.inner(),
            json!({
                "inner_key1": "inner_value1",
                "inner_key2": "inner_value2"
            })
        );
    }

    #[tokio::test]
    async fn test_tree_at() {
        let tree = Tree::new(value(), vec![]).get("key3").unwrap();
        let tree = tree.at(1).unwrap();
        assert_eq!(tree.inner(), serde_json::json!(2));
    }

    #[tokio::test]
    async fn test_tree_insert() {
        let mut tree = Tree::new(value(), vec![]);
        tree.insert("new_key", "new_value").unwrap();
        let tree = tree.get("new_key").unwrap();
        assert_eq!(tree.inner(), serde_json::json!("new_value"));
    }

    #[tokio::test]
    async fn test_push() {
        let mut tree = Tree::new(value(), vec![]).get("key3").unwrap();
        tree.push(10).unwrap();
        assert!(tree
            .inner()
            .as_array()
            .unwrap()
            .contains(&serde_json::json!(10)));
    }
    #[tokio::test]
    async fn test_tree_for_each() {
        let mut tree = Tree::new(value(), vec![]).get("key3").unwrap();
        tree.for_each(|v| {
            *v = Value::from(v.as_i64().unwrap() + 2i64);
        })
        .unwrap();
        assert_eq!(tree.inner(), json!([3, 4, 5]));
    }

    #[tokio::test]
    async fn test_tree_len() {
        let tree = Tree::new(value(), vec![]).get("key3").unwrap();
        assert_eq!(tree.len().unwrap(), 3);
    }
}
