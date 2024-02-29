#![allow(unused_variables)]
use anyhow::Result;
use nanodb::nanodb::NanoDB;
use serde::{Deserialize, Serialize};
use serde_json::{Map, Value};

#[allow(dead_code)]
#[tokio::main]
async fn main() -> Result<()> {
    let mut db = NanoDB::open("examples/data.json")?;

    // Setting
    db.insert("age", 40)?;
    db.insert("email", "johndoe@gmail.com")?;
    db.insert("fruits", vec!["apple", "banana", "orange"])?;
    db.insert("hobbies", vec!["ski", "tennis", "fitness", "climbing"])?;
    db.write()?;

    // Simple getter
    let age: i64 = db.get("age")?.into()?;
    let city: String = db.get("address")?.get("city")?.into()?;
    let fruits_value_tree: String = db.get("fruits")?.at(1)?.into()?;
    let address: Map<String, Value> = db.get("address")?.into()?;

    // Tree methods
    let number_of_fruits = db.get("fruits")?.array_len()?;
    let fruits = db.get("fruits")?.array_push("mango")?;
    let numbers = db
        .get("numbers")?
        .array_for_each(|v| {
            *v = Value::from(v.as_i64().unwrap() + 2i64);
        })
        .unwrap();
    db.merge_and_write(numbers)?;

    // Merge
    let fruits = db.get("fruits")?.array_push("coconut")?;
    db.merge(fruits)?;
    let address = db.get("address")?.insert("zip", "12345")?;
    db.merge(address)?;
    db.write()?;

    Ok(())
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Q {
    item: String,
    quantity: i32,
}
