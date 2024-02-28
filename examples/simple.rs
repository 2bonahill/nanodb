#![allow(unused_variables)]
use anyhow::Result;
use nanodb::{nanodb::NanoDB, tree::Tree};
use serde::{Deserialize, Serialize};
use serde_json::{Map, Value};

#[allow(dead_code)]
#[tokio::main]
async fn main() -> Result<()> {
    let mut db = NanoDB::new("data.json")?;

    // Simple getter
    let age: i64 = db.get("age")?.into()?;
    let city: String = db.get("address")?.get("city")?.into()?;
    let fruits_value_tree: String = db.get("fruits")?.at(1)?.into()?;

    // Setting
    db.insert("age", 40)?;
    db.insert("email", "johndoe@gmail.com")?;
    db.insert("fruits", vec!["apple", "banana", "orange", "avocado"])?;
    db.insert("hobbies", vec!["ski", "tennis", "fitness", "climbing"])?;

    // Tree methods
    let number_of_fruits = db.get("fruits")?.array_len()?;
    let fruits = db.get("fruits")?.array_push("mango")?;
    dbg!(fruits);

    let fruits_value_tree: Tree = db.get("fruits")?.at(1)?;
    let address: Map<String, Value> = db.get("address")?.into()?;

    // value trees
    let mut numbers: Tree = db.get("numbers")?;

    let numbers = numbers.array_for_each(|v| {
        *v = Value::from(v.as_i64().unwrap() * 2i64);
    });

    db.write()?;

    Ok(())
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Q {
    item: String,
    quantity: i32,
}
