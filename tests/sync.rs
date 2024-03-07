#![allow(unused_variables)]

use nanodb::{error::NanoDBError, nanodb::NanoDB};
use serde_json::{Map, Value};

#[tokio::test]
async fn sync_tests() -> Result<(), NanoDBError> {
    let mut db = NanoDB::open("examples/data/data.json")?;

    // Insert
    db.insert("age", 60).await?;
    db.insert("email", "johndoe@gmail.com").await?;
    db.insert("fruits", vec!["apple", "banana", "orange"])
        .await?;
    db.insert("hobbies", vec!["ski", "tennis", "fitness", "climbing"])
        .await?;
    db.write().await?;

    // Get
    let age: i64 = db.data().await.get("age")?.into()?;
    let city: String = db.data().await.get("address")?.get("city")?.into()?;
    let fruits_value_tree: String = db.data().await.get("fruits")?.at(1)?.into()?;
    let address: Map<String, Value> = db.data().await.get("address")?.into()?;

    // Tree methods
    let number_of_fruits = db.data().await.get("fruits")?.len()?;
    let fruits = db.data().await.get("fruits")?.push("mango")?;
    let numbers = db
        .data()
        .await
        .get("numbers")?
        .for_each(|v| {
            *v = Value::from(v.as_i64().unwrap() + 2i64);
        })
        .unwrap();
    db.merge_and_write(numbers).await?;

    // Merge
    let fruits = db.data().await.get("fruits")?.push("coconut")?;
    db.merge(fruits).await?;
    let address = db.data().await.get("address")?.insert("zip", "12345")?;
    db.merge(address).await?;
    db.write().await?;

    // Atomic reader
    let db = NanoDB::open("examples/data/data.json")?;
    let fruits: Vec<String> = db.read().await.get("fruits")?.into()?;
    let fruit_at_position_0: String = db.read().await.get("fruits")?.at(0)?.into()?;

    // Atomic writer
    let mut db = NanoDB::open("examples/data/data.json")?;
    db.update().await.insert("writer", "hi from writer")?;
    db.update()
        .await
        .get("address")?
        .insert("address-hi", "for address: hi from writer")?;
    db.write().await?;
    db.update().await.get("numbers")?.for_each(|v| {
        *v = Value::from(v.as_i64().unwrap() + 2i64);
    })?;
    db.write().await?;

    Ok(())
}
