//! # NanoDB
//! NanoDB is a simple, lightweight, and easy-to-use JSON database for Rust. It is designed to be used in small to medium-sized projects where a full-fledged database is not necessary. It is built on top of the `serde_json` crate and provides a simple and easy-to-use API for reading and writing JSON data.
//! ## Examples
//! ```rust

//! // let mut db = NanoDB::open("/path/to/data.json")?;
//! # use nanodb::nanodb::NanoDB;
//! # use serde_json::json;
//! # use serde_json::Value;
//! # use serde::{Deserialize, Serialize};
//! # let mut db = NanoDB::doctest_new_from(
//! #    "/path/to/file.json",
//! #    r#"{"key": {"nested_key": "nested_value"}}"#,
//! # )
//! # .unwrap();
//! db.insert("age", 40).unwrap();
//! db.insert("email", "johndoe@gmail.com").unwrap();
//! db.insert("fruits", vec!["apple", "banana", "orange"]).unwrap();
//! db.insert("hobbies", vec!["ski", "tennis", "fitness", "climbing"]).unwrap();
//! db.insert("address", json!({"city": "Zurich", "country": "Switzerland"})).unwrap();
//! db.insert("numbers", [1,2,3,4]).unwrap();
//! // db.write().unwrap(); // In case data needs to be written to the file
//!
//! // Simple getter
//! let age: i64 = db.get("age").unwrap().into().unwrap();
//! let city: String = db.get("address").unwrap().get("city").unwrap().into().unwrap();
//! let fruits_value_tree: String = db.get("fruits").unwrap().at(1).unwrap().into().unwrap();
//!
//! // Using a custom address struct
//! #[derive(Debug, Deserialize, Serialize)]
//! struct Address {
//!     city: String,
//!     country: String,
//! }
//! let address: Address = db.get("address").unwrap().into().unwrap();
//!
//! // Tree method (a tree consists of a part of the JSON object contained in the database)
//! let number_of_fruits = db.get("fruits").unwrap().len().unwrap();
//! let fruits = db.get("fruits").unwrap().push("mango").unwrap();
//! let numbers = db
//!     .get("numbers").unwrap()
//!     .for_each(|v| {
//!         *v = Value::from(v.as_i64().unwrap() + 2i64);
//!     })
//!     .unwrap();
//! // db.merge(numbers).unwrap();
//!
//! // Merge (after manipulation, the tree can be merged back into the database)
//! let fruits = db.get("fruits").unwrap().push("coconut").unwrap();
//! // db.merge(fruits).unwrap();
//! let address = db.get("address").unwrap().insert("zip", "12345").unwrap();
//! db.merge(address).unwrap();
//! // db.write().unwrap();
//! ```

pub mod error;
pub mod nanodb;
pub mod nanodb_v2;
pub mod tree;
