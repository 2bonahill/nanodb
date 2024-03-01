#![allow(unused_variables)]

use nanodb::{error::NanoDBError, nanodb::NanoDB};
use serde::{Deserialize, Serialize};
use serde_json::{Map, Value};

#[allow(dead_code)]
#[tokio::main]
async fn main() -> Result<(), NanoDBError> {
    single_threading().await?;
    multi_threading().await?;
    Ok(())
}

async fn single_threading() -> Result<(), NanoDBError> {
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
    let number_of_fruits = db.get("fruits")?.len()?;
    let fruits = db.get("fruits")?.push("mango")?;
    let numbers = db
        .get("numbers")?
        .for_each(|v| {
            *v = Value::from(v.as_i64().unwrap() + 2i64);
        })
        .unwrap();
    db.merge_and_write(numbers)?;

    // Merge
    let fruits = db.get("fruits")?.push("coconut")?;
    db.merge(fruits)?;
    let address = db.get("address")?.insert("zip", "12345")?;
    db.merge(address)?;
    db.write()?;

    Ok(())
}

async fn multi_threading() -> Result<(), NanoDBError> {
    let db = NanoDB::open("examples/data.json")?;

    let mut handles = Vec::new();

    // Standard Threads
    for i in 0..1000 {
        let mut db_clone = db.clone();
        let handle = std::thread::spawn(move || {
            // db.insert("age", i).unwrap();
            let mut fruits = db_clone.get("fruits").unwrap();
            fruits.push("hallo").unwrap();
            db_clone.merge(fruits).unwrap();
            db_clone.write().unwrap();
        });
        handles.push(handle);
    }

    // Await all tasks to complete
    for handle in handles {
        handle.join().unwrap();
    }

    println!("All threads have completed.");

    Ok(())
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Q {
    item: String,
    quantity: i32,
}
