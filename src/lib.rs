//! # NanoDB
//! NanoDB is a simple, lightweight, and easy-to-use JSON database for Rust. It is designed to be used in small to medium-sized projects where a full-fledged database is not necessary. It is built on top of the `serde_json` crate and provides a simple and easy-to-use API for reading and writing JSON data.
//! ## Examples
//! ```rust,ignore

//! let mut db = NanoDB::new("/path/to/data.json")?;
//!	// Setting
//! db.insert("age", 40)?;
//! db.insert("email", "johndoe@gmail.com")?;
//! db.insert("fruits", vec!["apple", "banana", "orange"])?;
//! db.insert("hobbies", vec!["ski", "tennis", "fitness", "climbing"])?;
//! db.write()?;
//!
//! // Simple getter
//! let age: i64 = db.get("age")?.into()?;
//! let city: String = db.get("address")?.get("city")?.into()?;
//! let fruits_value_tree: String = db.get("fruits")?.at(1)?.into()?;
//! let address: Map<String, Value> = db.get("address")?.into()?;
//!
//! // Tree method (a tree consists of a part of the JSON object contained in the database)
//! let number_of_fruits = db.get("fruits")?.array_len()?;
//! let fruits = db.get("fruits")?.array_push("mango")?;
//! let numbers = db
//!     .get("numbers")?
//!     .array_for_each(|v| {
//!         *v = Value::from(v.as_i64().unwrap() + 2i64);
//!     })
//!     .unwrap();
//! db.merge_and_write(numbers)?;
//!
//! // Merge (after manipulation, the tree can be merged back into the database)
//! let fruits = db.get("fruits")?.array_push("coconut")?;
//! db.merge(fruits)?;
//! let address = db.get("address")?.insert("zip", "12345")?;
//! db.merge(address)?;
//! db.write()?;
//! ```

pub mod error;
pub mod nanodb;
pub mod tree;
