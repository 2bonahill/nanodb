use anyhow::Result;

use nanodb::nanodb::NanoDB;
use serde::{Deserialize, Serialize};
use serde_json::{Map, Value};

#[tokio::main]
async fn main() -> Result<()> {
    let mut db = NanoDB::new("data.json")?;

    // Set and get a string
    db.insert("age", 40)?;
    db.insert("email", "johndoe@gmail.com")?;

    let _city_name: String = db.get("address")?.get("city")?.into()?;

    let _second_fruit: String = db.get("fruits")?.at(1)?.into()?;

    let _address: Map<String, Value> = db.get("address")?.into()?;

    let city_value_wrapper = db.get("address")?.get("ziparray")?.at(3)?;
    dbg!(city_value_wrapper);

    Ok(())
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Q {
    item: String,
    quantity: i32,
}
