#![allow(unused_variables)]

use nanodb::{error::NanoDBError, nanodb::NanoDB};
use serde::{Deserialize, Serialize};
use serde_json::{Map, Value};

#[allow(dead_code)]
#[tokio::main]
async fn main() -> Result<(), NanoDBError> {
    let mut db = NanoDB::open("examples/data.json")?;

    // Insert
    db.insert("age", 60).await?;
    db.insert("email", "johndoe@gmail.com").await?;
    db.insert("fruits", vec!["apple", "banana", "orange"])
        .await?;
    db.insert("hobbies", vec!["ski", "tennis", "fitness", "climbing"])
        .await?;
    db.write().await?;

    // Get
    let age: i64 = db.data().await?.get("age")?.into()?;
    let city: String = db.data().await?.get("address")?.get("city")?.into()?;
    let fruits_value_tree: String = db.data().await?.get("fruits")?.at(1)?.into()?;
    let address: Map<String, Value> = db.data().await?.get("address")?.into()?;

    // // Tree methods
    let number_of_fruits = db.data().await?.get("fruits")?.len()?;
    let fruits = db.data().await?.get("fruits")?.push("mango")?;
    let numbers = db
        .data()
        .await?
        .get("numbers")?
        .for_each(|v| {
            *v = Value::from(v.as_i64().unwrap() + 2i64);
        })
        .unwrap();
    db.merge_and_write(numbers).await?;

    // // Merge
    let fruits = db.data().await?.get("fruits")?.push("coconut")?;
    db.merge(fruits).await?;
    let address = db.data().await?.get("address")?.insert("zip", "12345")?;
    db.merge(address).await?;
    db.write().await?;

    // Atomic reader
    let db = NanoDB::open("examples/data.json")?;
    let fruits: Vec<String> = db.read().await?.get("fruits")?.into()?;
    let fruit_at_position_0: String = db.read().await?.get("fruits")?.at(0)?.into()?;
    // dbg!((&fruits, &fruit_at_position_0));

    // Atomic writer
    let mut db = NanoDB::open("examples/data.json")?;
    db.update().await?.insert("writer", "hi from writer")?;
    db.update()
        .await?
        .get("address")?
        .insert("writer", "for address: hi from writer")?;
    dbg!(&db);
    // db.write().await?;

    Ok(())
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Q {
    item: String,
    quantity: i32,
}