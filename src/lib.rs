//! # NanoDB
//! NanoDB is a simple, lightweight, and easy-to-use JSON database for Rust. It is designed to be used in small to medium-sized projects where a full-fledged database is not necessary. It is built on top of the serde_json crate and provides a simple and easy-to-use API for reading and writing JSON data.
//! NanoDB provides the following features:
//! * **Simple API**: NanoDB provides a simple and easy-to-use API for reading and writing JSON data.
//! * **Atomic Reads and Writes**: NanoDB provides atomic reads and writes, ensuring that the data is always consistent.
//! * **Tree Methods**: NanoDB provides tree methods for working with JSON data, such as getting, setting, and merging data.
//! * **Error Handling**: NanoDB provides comprehensive error handling, ensuring that errors are handled gracefully.
//! * **Asynchronous I/O**: NanoDB uses asynchronous I/O to ensure that it is fast and efficient.
//! * **Lightweight**: NanoDB is lightweight and has minimal dependencies, making it easy to use in small to medium-sized projects.
//! * **JSON Serialization and Deserialization**: NanoDB uses the serde_json crate for JSON serialization and deserialization, ensuring that it is fast and efficient.
//! * **Thread Safety**: NanoDB is thread-safe, ensuring that it can be used in multi-threaded applications.

//! ## Trees
//! NanoDB know three different types of trees:
//! * **Tree**: A struct representing a read-only tree. This struct contains a clone of the DB's JSON value and a path. The JSON value is the actual data of the tree, and the path is the path to the current location in the tree.
//! * **ReadGuardedTree**: A struct representing a read-guarded tree. This struct contains a read lock guard and a tree. The read lock guard ensures that the tree cannot be modified by other threads while it is being read.
//! * **WriteGuardedTree**: A struct representing a write-guarded tree. This struct contains a write lock guard and a tree. The write lock guard ensures that the tree cannot be modified by other threads while it is being written to.
//!
//! This construct allows **any number of read guarded trees** or **at most one write guarded tree** at any point in time.
//! ## Examples
//! ```rust,no_run
//! use nanodb::{error::NanoDBError, nanodb::NanoDB, trees::tree::Tree};
//! use serde_json::{Map, Value};
//!
//! #[tokio::main]
//! async fn main() -> Result<(), NanoDBError> {
//!     let mut db = NanoDB::open("examples/data/data.json")?;
//!     assert!(false);
//!     // Atomic Updates
//!     db.insert("age", 42).await?;
//!     db.insert("fruits", vec!["apple", "banana"]).await?;
//!     db.update().await.get("address")?.insert("key", "value")?;
//!     db.update().await.get("hobbies")?.push("reading")?;
//!     db.update().await.get("numbers")?.for_each(|v| {
//!         *v = Value::from(v.as_i64().unwrap() + 2i64);
//!     })?;
//!		db.update().await.remove("age")?;
//!		db.insert("age", 42).await?;
//!
//!     db.write().await?;
//!
//!     // Simple reading (working with a cloned sub-tree)
//!     let age: i64 = db.data().await.get("age")?.into()?;
//!     let city: String = db.data().await.get("address")?.get("city")?.into()?;
//!     let fruit: String = db.data().await.get("fruits")?.at(1)?.into()?;
//!     let address: Map<String, Value> = db.data().await.get("address")?.into()?;
//!
//!     // Atomic reader
//!     let fruits: Vec<String> = db.read().await.get("fruits")?.into()?;
//!     let first_fruit: String = db.read().await.get("fruits")?.at(0)?.into()?;
//!
//!     // Tree methods and merging
//!     let mut address_tree: Tree = db.data().await.get("address")?;
//!     address_tree.insert("zip", "12345")?;
//!     db.insert_tree(address_tree).await?;
//!     let numbers = db
//!         .data()
//!         .await
//!         .get("numbers")?
//!         .for_each(|v| {
//!             *v = Value::from(v.as_i64().unwrap() + 2i64);
//!         })
//!         .unwrap();
//!     db.insert_tree(numbers).await?;
//!     db.write().await?;
//!
//!     Ok(())
//! }
//! ```

pub mod error;
pub mod nanodb;
pub mod trees;
