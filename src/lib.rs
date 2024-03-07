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
//! ## Usage
//! To use NanoDB, add the following to your Cargo.toml file:
//! ```toml
//! [dependencies]
//! nanodb = "0.1.0"
//! ```

//! ## Trees
//! NanoDB know three different types of trees:
//! * **Tree**: A struct representing a read-only tree. This struct contains a clone of the DB's JSON value and a path. The JSON value is the actual data of the tree, and the path is the path to the current location in the tree.
//! * **ReadGuardedTree**: A struct representing a read-guarded tree. This struct contains a read lock guard and a tree. The read lock guard ensures that the tree cannot be modified by other threads while it is being read.
//! * **WriteGuardedTree**: A struct representing a write-guarded tree. This struct contains a write lock guard and a tree. The write lock guard ensures that the tree cannot be modified by other threads while it is being written to.

//! ## Examples
//! ```rust,no_run
//! use nanodb::{error::NanoDBError, nanodb::NanoDB};
//! use serde_json::{Map, Value};
//!
//! #[tokio::main]
//! async fn main() -> Result<(), NanoDBError> {
//!     let mut db = NanoDB::open("examples/data/data.json")?;
//!
//!     // Insert
//!     db.insert("age", 60).await?;
//!     db.insert("email", "johndoe@gmail.com").await?;
//!     db.insert("fruits", vec!["apple", "orange"]).await?;
//!     db.insert("hobbies", vec!["ski", "tennis"]).await?;
//!     db.write().await?;
//!
//!     // Get
//!     let age: i64 = db.data().await.get("age")?.into()?;
//!     let city: String = db.data().await.get("address")?.get("city")?.into()?;
//!     let fruits_value_tree: String = db.data().await.get("fruits")?.at(1)?.into()?;
//!     let address: Map<String, Value> = db.data().await.get("address")?.into()?;
//!
//!     // Tree methods
//!     let number_of_fruits = db.data().await.get("fruits")?.len()?;
//!     let fruits = db.data().await.get("fruits")?.push("mango")?;
//!     let numbers = db
//!         .data()
//!         .await
//!         .get("numbers")?
//!         .for_each(|v| {
//!             *v = Value::from(v.as_i64().unwrap() + 2i64);
//!         })
//!         .unwrap();
//!     db.merge_and_write(numbers).await?;
//!
//!     // Merge
//!     let fruits = db.data().await.get("fruits")?.push("coconut")?;
//!     db.merge_from(fruits).await?;
//!     let address = db.data().await.get("address")?.insert("zip", "12345")?;
//!     db.merge_from(address).await?;
//!     db.write().await?;
//!
//!     // Atomic reader
//!     let db = NanoDB::open("examples/data/data.json")?;
//!     let fruits: Vec<String> = db.read().await.get("fruits")?.into()?;
//!     let fruit_at_position_0: String = db.read().await.get("fruits")?.at(0)?.into()?;
//!
//!     // Atomic writer
//!     let mut db = NanoDB::open("examples/data/data.json")?;
//!     db.update().await.insert("writer", "hi from writer")?;
//!     db.update()
//!         .await
//!         .get("address")?
//!         .insert("address-hi", "for address: hi from writer")?;
//!     db.write().await?;
//!     db.update().await.get("numbers")?.for_each(|v| {
//!         *v = Value::from(v.as_i64().unwrap() + 2i64);
//!     })?;
//!     db.write().await?;
//!
//!     Ok(())
//! }
//! ```

pub mod error;
pub mod nanodb;
pub mod trees;
