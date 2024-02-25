use anyhow::Result;
use nanodb::nanodb::NanoDB;
use serde::{Deserialize, Serialize};

#[tokio::main]
async fn main() -> Result<()> {
    let mut db = NanoDB::new("data.json")?;

    // Set and get a string
    db.insert("age", 40)?;
    db.insert("email", "johndoe@gmail.com")?;
    db.write()?;

    // Set and get a vector
    db.insert("items3", vec!["apple", "banana", "orange", "avocado"])?;
    db.write()?;

    db.array_push("items3", "grapes")?;
    db.array_push("items3", "strawberries")?;
    db.write()?;

    let mut items: Vec<String> = db.get("items2")?;
    items.push("grapes".to_string());
    items.push("strawberries".to_string());
    db.insert("items2", items)?;
    db.write()?;

    Ok(())
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Q {
    item: String,
    quantity: i32,
}
