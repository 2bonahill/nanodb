use anyhow::Result;
use nanodb::{nanodb::NanoDB, value_tree::ValueTree};
use serde::{Deserialize, Serialize};
use serde_json::{Map, Value};

#[allow(dead_code)]
#[tokio::main]
async fn main() -> Result<()> {
    let mut db = NanoDB::new("data.json")?;

    // Setting
    db.insert("age", 40)?;
    db.insert("email", "johndoe@gmail.com")?;
    db.insert("fruits", vec!["apple", "banana", "orange", "avocado"])?;
    db.insert("hobbies", vec!["ski", "tennis", "fitness", "climbing"])?;

    // getters
    let _city_name: String = db.get("address")?.get("city")?.into()?;
    let _fruits_value_tree: ValueTree = db.get("fruits")?.at(1)?;
    let _address: Map<String, Value> = db.get("address")?.into()?;

    // value trees
    let mut numbers: ValueTree = db.get("numbers")?;
    dbg!(&numbers);
    let numbers = numbers.for_each(|v| {
        *v = Value::from(v.as_i64().unwrap() * 2i64);
    });
    dbg!(&numbers);

    db.write()?;

    Ok(())
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Q {
    item: String,
    quantity: i32,
}
