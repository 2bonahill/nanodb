#![allow(unused_variables)]
#![allow(dead_code)]

use nanodb::{error::NanoDBError, nanodb::NanoDB, trees::tree::Tree};
use serde::Deserialize;
use serde_json::{json, Map, Value};

#[derive(Deserialize)]
pub struct MyStruct {
    name: String,
    versions: Vec<f64>,
}

#[tokio::main]
async fn main() -> Result<(), NanoDBError> {
    let json_data = r#"{
			"key1": "Welcome!",
			"key2": 42,
			"key3": {
				"name": "NanoDB",
				"versions": [1.0, 2.0, 3.0]
			},
			"key4": [1, 2, 3],
			"key5": ["Welcome", "to", "NanoDB"]
		}"#;

    let mut db = NanoDB::new_from("examples/data/data2.json", json_data).unwrap();

    // Basic reads (working with cloned sub-tree)
    let text: String = db.data().await.get("key1")?.into()?;
    let number: i64 = db.data().await.get("key2")?.into()?;
    let object: MyStruct = db.data().await.get("key3")?.into()?;
    let array: Vec<i64> = db.data().await.get("key4")?.into()?;

    // Basic inserts
    db.insert("age", 42).await?; // shorthand for db.update().await.insert("age", 42)?;
    db.insert("crates", vec!["tokio", "serde"]).await?;
    db.insert("some_map", Map::new()).await?;
    db.insert("person", json!({"name": "Donald"})).await?;
    db.write().await?; // write to file if needed

    // Atomic CRUD operations and advanced data manipulation
    db.update().await.remove("age")?;
    db.update().await.get("key3")?.insert("key", "value")?;
    db.update().await.get("key3")?.get("versions")?.push(42.0)?;
    db.update().await.get("key4")?.for_each(|v| {
        *v = Value::from(v.as_i64().unwrap() * 2i64);
    })?;
    db.write().await?; // write to file if needed

    // Atomic reader (read() returns a read lock on the db, so it's safe to use in async functions)
    let versions: Vec<String> = db.read().await.get("key5")?.into()?;
    let number: i64 = db.read().await.get("key4")?.at(0)?.into()?;

    // Tree methods and merging
    let mut my_tree: Tree = db.data().await.get("key3")?;
    my_tree.insert("language", "Rust")?;
    db.insert_tree(my_tree).await?;
    db.write().await?;

    // Advanced Write locks
    let mut write_lock = db.update().await;
    write_lock.insert("key1", "Welcome to NanoDB")?;
    write_lock.insert("key1", "Welcome to NanoDB again")?;
    write_lock.release_lock(); // release the lock manually to avoid deadlocks
    let mut another_write_lock = db.update().await;
    assert_eq!(
        another_write_lock.get("key1")?.into::<String>().unwrap(),
        "Welcome to NanoDB again"
    );

    Ok(())
}
