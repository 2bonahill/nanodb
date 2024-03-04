use crate::{error::NanoDBError, tree::PathStep};
use serde_json::Value;
use std::sync::Arc;
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
        match self {
            GuardedTree::ReadGuarded(tree) => {
                // Check if the key exists within the read guard's data.
                if tree.guard.get(key).is_none() {
                    return Err(NanoDBError::KeyNotFound(key.to_owned()));
                }
                // Key exists, update the path accordingly.
                tree.path.push(PathStep::Key(key.to_owned()));
            }
            GuardedTree::WriteGuarded(tree) => {
                // Check if the key exists within the write guard's data.
                if tree.guard.get(key).is_none() {
                    return Err(NanoDBError::KeyNotFound(key.to_owned()));
                }
                // Key exists, update the path accordingly.
                tree.path.push(PathStep::Key(key.to_owned()));
            }
        }
        // Return a mutable reference to self, now with the updated path.
        Ok(self)
    }

    fn at(&mut self, index: usize) -> Result<&mut Self, NanoDBError> {
        dbg!("hey there at");
        match self {
            GuardedTree::ReadGuarded(tree) => {
                if tree.guard.is_array() {
                    // Check if the index exists within the read guard's data.
                    if tree.guard.get(index).is_none() {
                        return Err(NanoDBError::IndexOutOfBounds(index));
                    }
                    // Index exists, update the path accordingly.
                    tree.path.push(PathStep::Index(index));
                } else {
                    return Err(NanoDBError::NotAnArray("at".to_owned()));
                }
            }
            GuardedTree::WriteGuarded(tree) => {
                dbg!(&tree.guard);
                if tree.guard.is_array() {
                    // Check if the index exists within the read guard's data.
                    if tree.guard.get(index).is_none() {
                        return Err(NanoDBError::IndexOutOfBounds(index));
                    }
                    // Index exists, update the path accordingly.
                    tree.path.push(PathStep::Index(index));
                } else {
                    return Err(NanoDBError::NotAnArray("at".to_owned()));
                }
            }
        }
        // Return a mutable reference to self, now with the updated path.
        Ok(self)
    }
}
