use crate::{error::NanoDBError, tree::PathStep};
use serde::Deserialize;
use serde_json::Value;
use tokio::sync::{RwLockReadGuard, RwLockWriteGuard}; // Assuming NanoDBError is defined elsewhere in your crate

// Define the ReadGuardedTree struct
#[derive(Debug)]
pub struct ReadGuardedTree<'a> {
    guard: RwLockReadGuard<'a, Value>,
    path: Vec<PathStep>,
}

// Define the WriteGuardedTree struct
#[derive(Debug)]
pub struct WriteGuardedTree<'a> {
    guard: RwLockWriteGuard<'a, Value>,
    path: Vec<PathStep>,
}

// Define the GuardedTree enum with the new structs
#[derive(Debug)]
pub enum GuardedTree<'a> {
    ReadGuarded(ReadGuardedTree<'a>),
    WriteGuarded(WriteGuardedTree<'a>),
}

pub trait GuardedTreeOps<'a> {
    fn get(&mut self, key: &str) -> Result<&mut Self, NanoDBError>;
    fn at(&mut self, index: usize) -> Result<&mut Self, NanoDBError>;
    fn into<T: for<'de> Deserialize<'de>>(self) -> Result<T, NanoDBError>;
}

impl<'a> ReadGuardedTree<'a> {
    // Constructor for a ReadGuardedTree
    pub fn new(guard: RwLockReadGuard<'a, Value>, path: Vec<PathStep>) -> Self {
        ReadGuardedTree { guard, path }
    }

    // Implement methods specific to ReadGuardedTree here
}

impl<'a> WriteGuardedTree<'a> {
    // Constructor for a WriteGuardedTree
    pub fn new(guard: RwLockWriteGuard<'a, Value>, path: Vec<PathStep>) -> Self {
        WriteGuardedTree { guard, path }
    }

    // Implement methods specific to WriteGuardedTree here
}

impl GuardedTreeOps<'_> for GuardedTree<'_> {
    fn get(&mut self, key: &str) -> Result<&mut Self, NanoDBError> {
        // check if the key exists within the read guard's data.
        // for this, traverse the pathsteps (in self.path) plus the new key.
        // if the key exists, update the path accordingly.
        let (data, path) = match self {
            GuardedTree::ReadGuarded(tree) => (tree.guard.clone(), tree.path.clone()),
            GuardedTree::WriteGuarded(tree) => (tree.guard.clone(), tree.path.clone()),
        };

        let new_path_step = PathStep::Key(key.to_owned());

        new_path_is_valid(&data, &path, &new_path_step)?;

        match self {
            GuardedTree::ReadGuarded(tree) => {
                // Key exists, update the path accordingly.
                tree.path.push(new_path_step);
            }
            GuardedTree::WriteGuarded(tree) => {
                // Key exists, update the path accordingly.
                tree.path.push(new_path_step);
            }
        }
        // Return a mutable reference to self, now with the updated path.
        Ok(self)
    }

    fn at(&mut self, index: usize) -> Result<&mut Self, NanoDBError> {
        // check if the key exists within the read guard's data.
        // for this, traverse the pathsteps (in self.path) plus the new key.
        // if the key exists, update the path accordingly.
        let (data, path) = match self {
            GuardedTree::ReadGuarded(tree) => (tree.guard.clone(), tree.path.clone()),
            GuardedTree::WriteGuarded(tree) => (tree.guard.clone(), tree.path.clone()),
        };

        let new_path_step = PathStep::Index(index);

        new_path_is_valid(&data, &path, &new_path_step)?;

        match self {
            GuardedTree::ReadGuarded(tree) => {
                // Key exists, update the path accordingly.
                tree.path.push(new_path_step);
            }
            GuardedTree::WriteGuarded(tree) => {
                // Key exists, update the path accordingly.
                tree.path.push(new_path_step);
            }
        }
        // Return a mutable reference to self, now with the updated path.
        Ok(self)
    }

    fn into<T: for<'de> Deserialize<'de>>(self) -> Result<T, NanoDBError> {
        let (data, path) = match self {
            GuardedTree::ReadGuarded(tree) => (tree.guard.clone(), tree.path.clone()),
            GuardedTree::WriteGuarded(tree) => (tree.guard.clone(), tree.path.clone()),
        };

        // get the right json field according to the path steps in path
        let data = path.iter().fold(data, |acc, step| match step {
            PathStep::Key(k) => acc.get(k).unwrap().clone(),
            PathStep::Index(i) => acc.get(*i).unwrap().clone(),
        });
        serde_json::from_value(data).map_err(Into::into)
    }
}

fn new_path_is_valid(
    data: &Value,
    path: &[PathStep],
    new_step: &PathStep,
) -> Result<bool, NanoDBError> {
    let mut current = data;
    for step in path {
        match step {
            PathStep::Key(k) => {
                if let Some(new_data) = current.get(k) {
                    current = new_data;
                } else {
                    return Err(NanoDBError::InvalidJSONPath);
                }
            }
            PathStep::Index(i) => {
                if let Some(new_data) = current.get(*i) {
                    dbg!(new_data);
                    current = new_data;
                } else {
                    return Err(NanoDBError::InvalidJSONPath);
                }
            }
        }
    }
    match new_step {
        PathStep::Key(k) => {
            if current.get(k).is_none() {
                return Err(NanoDBError::InvalidJSONPath);
            };
        }
        PathStep::Index(idx) => {
            if !current.is_array() {
                return Err(NanoDBError::NotAnArray("xx".to_owned())); // TODO: better error message
            }
            if current.get(idx).is_none() {
                return Err(NanoDBError::IndexOutOfBounds(*idx));
            }
        }
    }
    Ok(true)
}
