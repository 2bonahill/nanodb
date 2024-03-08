#![allow(unused_variables)]

use nanodb::{error::NanoDBError, nanodb::NanoDB, trees::tree::Tree};
use serde_json::{Map, Value};

#[tokio::main]
async fn main() -> Result<(), NanoDBError> {
    let mut db = NanoDB::open("examples/data/data.json")?;

    // Atomic Updates
    db.insert("age", 42).await?;
    db.insert("fruits", vec!["apple", "banana"]).await?;
    db.update().await.get("address")?.insert("key", "value")?;
    db.update().await.get("hobbies")?.push("reading")?;
    db.update().await.get("numbers")?.for_each(|v| {
        *v = Value::from(v.as_i64().unwrap() + 2i64);
    })?;
    db.write().await?;

    // Simple reading (working with a cloned sub-tree)
    let age: i64 = db.data().await.get("age")?.into()?;
    let city: String = db.data().await.get("address")?.get("city")?.into()?;
    let fruit: String = db.data().await.get("fruits")?.at(1)?.into()?;
    let address: Map<String, Value> = db.data().await.get("address")?.into()?;

    // Atomic reader
    let fruits: Vec<String> = db.read().await.get("fruits")?.into()?;
    let first_fruit: String = db.read().await.get("fruits")?.at(0)?.into()?;

    // Tree methods and merging
    let mut address_tree: Tree = db.data().await.get("address")?;
    address_tree.insert("zip", "12345")?;
    db.insert_tree(address_tree).await?;
    let numbers = db
        .data()
        .await
        .get("numbers")?
        .for_each(|v| {
            *v = Value::from(v.as_i64().unwrap() + 2i64);
        })
        .unwrap();
    db.insert_tree(numbers).await?;
    db.write().await?;

    Ok(())
}
