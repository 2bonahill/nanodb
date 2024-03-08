#![allow(unused_variables)]

use nanodb::{error::NanoDBError, nanodb::NanoDB};
use serde_json::{Map, Value};

#[tokio::test]
async fn sync_tests() -> Result<(), NanoDBError> {
    let mut db = NanoDB::open("examples/data/data.json")?;

    // Insert
    db.insert("age", 60).await?;
    assert_eq!(db.data().await.get("age")?.into::<i64>()?, 60);
    db.insert("email", "johndoe@gmail.com").await?;
    assert_eq!(
        db.data().await.get("email")?.into::<String>()?,
        "johndoe@gmail.com"
    );
    db.insert("fruits", vec!["apple", "banana"]).await?;
    assert_eq!(
        db.data().await.get("fruits")?.into::<Vec<String>>()?,
        vec!["apple", "banana"]
    );
    db.insert("hobbies", vec!["ski", "tennis"]).await?;
    assert_eq!(
        db.data().await.get("hobbies")?.into::<Vec<String>>()?,
        vec!["ski", "tennis"]
    );
    db.write().await?;

    // Get
    let age: i64 = db.data().await.get("age")?.into()?;
    assert_eq!(age, 60);

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
    db.insert_tree(numbers).await?;

    // Merge
    let fruits = db.data().await.get("fruits")?.push("coconut")?;
    db.insert_tree(fruits).await?;
    let address = db.data().await.get("address")?.insert("zip", "12345")?;
    db.insert_tree(address).await?;
    db.write().await?;

    // Atomic reader
    let db = NanoDB::open("examples/data/data.json")?;
    let fruits: Vec<String> = db.read().await.get("fruits")?.into()?;
    let fruit_at_position_0: String = db.read().await.get("fruits")?.at(0)?.into()?;

    // Atomic writer
    let mut db = NanoDB::open("examples/data/data.json")?;
    db.update().await.insert("key", "value")?;
    assert_eq!(db.data().await.get("key")?.into::<String>()?, "value");
    db.update().await.get("address")?.insert("key", "value")?;
    db.write().await?;
    db.update().await.get("numbers")?.for_each(|v| {
        *v = Value::from(v.as_i64().unwrap() + 2i64);
    })?;
    db.update().await.get("hobbies")?.push("reading")?;
    assert_eq!(
        db.data().await.get("hobbies")?.into::<Vec<String>>()?,
        vec!["ski", "tennis", "reading"]
    );
    db.write().await?;

    // remove
    db.insert("age", 60).await?;
    db.update().await.remove("age")?;
    assert!(db.data().await.get("age").is_err());

    Ok(())
}
