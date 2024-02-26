use anyhow::Result;
use nanodb::nanodb::NanoDB;
use serde::{Deserialize, Serialize};
use serde_json::Value;

#[allow(dead_code)]
#[tokio::main]
async fn main() -> Result<()> {
    let mut db = NanoDB::new("data.json")?;

    // Set and get a string
    db.insert("age", 40)?;
    db.insert("email", "johndoe@gmail.com")?;

    // Set and get a vector
    db.insert("fruits", vec!["apple", "banana", "orange", "avocado"])?;

    db.array_push("fruits", "grapes")?;
    db.array_push("fruits", "strawberries")?;
    db.nested_array_push(&["fruits"], "mango")?;
    db.nested_array_push(&["address", "ziparray"], "666")?;
    db.write()?;

    let mut items: Vec<String> = db.get("fruits")?.into()?;
    items.push("grapes".to_string());
    items.push("strawberries".to_string());
    db.insert("fruits2", items)?;

    db.array_for_each("numbers", |v| {
        *v = Value::from(v.as_i64().unwrap() + 1i64);
    })?;

    db.write()?;

    Ok(())
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Q {
    item: String,
    quantity: i32,
}
