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
    db.insert("items2", vec!["apple", "banana", "orange"])?;
    db.write()?;

    let items: Vec<Q> = db.get("quantities")?;
    dbg!(&items);

    Ok(())
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Q {
    item: String,
    quantity: i32,
}
