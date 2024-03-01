#![allow(unused_variables)]

use nanodb::{error::NanoDBError, nanodb_v2::NanoDB};
use serde::{Deserialize, Serialize};
use serde_json::{Map, Value};

#[allow(dead_code)]
#[tokio::main]
async fn main() -> Result<(), NanoDBError> {
    let mut db = NanoDB::open("examples/data.json")?;

    // Setting
    db.insert("age", 60).await?;
    db.insert("email", "johndoe@gmail.com").await?;
    db.insert("fruits", vec!["apple", "banana", "orange"])
        .await?;
    db.insert("hobbies", vec!["ski", "tennis", "fitness", "climbing"])
        .await?;
    db.write().await?;

    // // Simple getter
    let age: i64 = db.get("age").await?.into()?;
    dbg!(age);
    let city: String = db.get("address").await?.get("city")?.into()?;
    let fruits_value_tree: String = db.get("fruits").await?.at(1)?.into()?;
    let address: Map<String, Value> = db.get("address").await?.into()?;

    // // Tree methods
    let number_of_fruits = db.get("fruits").await?.len()?;
    let fruits = db.get("fruits").await?.push("mango")?;
    let numbers = db
        .get("numbers")
        .await?
        .for_each(|v| {
            *v = Value::from(v.as_i64().unwrap() + 2i64);
        })
        .unwrap();
    db.merge_and_write(numbers).await?;

    // // Merge
    let fruits = db.get("fruits").await?.push("coconut")?;
    db.merge(fruits).await?;
    let address = db.get("address").await?.insert("zip", "12345")?;
    db.merge(address).await?;
    db.write().await?;

    Ok(())
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Q {
    item: String,
    quantity: i32,
}
