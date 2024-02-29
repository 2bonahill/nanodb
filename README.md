# NanoDB
Simple lightweight, pure-rust, high-performance and easy to use local JSON database.

## What is NanoDB 
NanoDB is a simple, lightweight, and easy-to-use JSON database for Rust. It is designed to be used in small to medium-sized projects where a full-fledged database is not necessary. It is built on top of the `serde_json` crate and provides a simple and easy-to-use API for reading and writing JSON data.

## Examples
```rust
use nanodb::nanodb::NanoDB;

// Init a new database
let mut db = NanoDB::new("/path/to/data.json")?;

// In memory data manipulation
db.insert("age", 40)?;
db.insert("email", "johndoe@gmail.com")?;
db.insert("fruits", vec!["apple", "banana", "orange"])?;
db.insert("hobbies", vec!["ski", "tennis", "fitness", "climbing"])?;
db.insert("address", json!({"city": "New York"}))?;
db.insert("numbers", [1,2,3,4])?;

// Write data to the file
db.write()?; 

// Simple getter
let age: i64 = db.get("age")?.into()?;
let city: String = db.get("address")?.get("city")?.into()?;
let fruits_value_tree: String = db.get("fruits")?.at(1)?.into()?;
let address: Map<String, Value> = db.get("address")?.into()?;

// Tree method (a tree consists of a part of the JSON object contained in the database)
let number_of_fruits = db.get("fruits")?.array_len()?;
let fruits = db.get("fruits")?.array_push("mango")?;
let numbers = db
    .get("numbers")?
    .array_for_each(|v| {
        *v = Value::from(v.as_i64()? + 2i64);
    })?;

// Merge (after manipulation, the tree can be merged back into the database)
db.merge(numbers)?; 

let fruits = db.get("fruits")?.array_push("coconut")?;
db.merge(fruits)?;

let address = db.get("address")?.insert("zip", "12345")?;
db.merge_and_write(address)?;
```