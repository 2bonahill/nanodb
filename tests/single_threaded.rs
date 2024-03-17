#![allow(unused_variables)]

use nanodb::{error::NanoDBError, nanodb::NanoDB};
use serde_json::{json, Map, Value};

#[tokio::test]
async fn sync_tests() -> Result<(), NanoDBError> {
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

    // Getters
    assert_eq!(db.data().await.get("key1")?.into::<String>()?, "Welcome!");
    assert_eq!(
        db.data().await.get("key3")?.get("name")?.into::<String>()?,
        "NanoDB"
    );
    assert_eq!(
        db.data()
            .await
            .get("key3")?
            .get("versions")?
            .at(0)?
            .into::<f32>()?,
        1.0
    );
    assert_eq!(db.data().await.get("key2")?.into::<i32>()?, 42);
    assert!(matches!(
        db.data().await.get("key0"),
        Err(NanoDBError::KeyNotFound(_))
    ));
    assert!(matches!(
        db.data().await.get("key3")?.get("versions")?.at(999),
        Err(NanoDBError::IndexOutOfBounds(_))
    ));
    assert!(
        matches!(
            db.data()
                .await
                .get("key3")?
                .get("versions")?
                .at(0)?
                .into::<String>(),
            Err(NanoDBError::TypeMismatch(_))
        ),
        "Type mismatch error expected"
    );
    assert_eq!(
        db.data().await.get("key3")?.into::<Map<String, Value>>()?,
        json!(
            {
                "name": "NanoDB",
                "versions": [1.0, 2.0, 3.0]
            }
        )
        .as_object()
        .unwrap()
        .to_owned()
    );

    // remove later
    let db2 = NanoDB::open("examples/data/data.json")?;

    // Insert
    db.insert("key6", 42).await?;
    assert_eq!(db.data().await.get("key6")?.into::<i64>()?, 42);
    db.insert("key7", "Rust").await?;
    assert_eq!(db.data().await.get("key7")?.into::<String>()?, "Rust");
    db.insert("key8", vec!["A", "B"]).await?;
    assert_eq!(
        db.data().await.get("key8")?.into::<Vec<String>>()?,
        vec!["A", "B"]
    );
    db.write().await?;

    // Tree methods
    let number_of_versions = db.data().await.get("key3")?.get("versions")?.len()?;
    let numbers = db
        .data()
        .await
        .get("key4")?
        .for_each(|v| {
            *v = Value::from(v.as_i64().unwrap() + 2i64);
        })
        .unwrap();
    db.insert_tree(numbers).await?;

    // Merge
    let versions = db.data().await.get("key3")?.get("versions")?.push(3.1)?;
    db.insert_tree(versions).await?;
    let info = db.data().await.get("key3")?.insert("language", "Rust")?;
    db.insert_tree(info).await?;

    // Atomic reader
    let db = NanoDB::new_from("examples/data/data2.json", json_data).unwrap();

    let welcome: Vec<String> = db.read().await.get("key5")?.into()?;
    assert_eq!(welcome, vec!["Welcome", "to", "NanoDB"]);
    let first_version: f64 = db
        .read()
        .await
        .get("key3")?
        .get("versions")?
        .at(0)?
        .into()?;
    assert_eq!(first_version, 1.0);

    // Atomic writer
    db.update().await.insert("key6", "value6")?;
    assert_eq!(db.data().await.get("key6")?.into::<String>()?, "value6");
    db.update().await.get("key3")?.insert("language", "Rust")?;
    assert_eq!(
        db.data()
            .await
            .get("key3")?
            .get("language")?
            .into::<String>()?,
        "Rust"
    );
    db.update().await.get("key4")?.for_each(|v| {
        *v = Value::from(v.as_i64().unwrap() + 2i64);
    })?;
    assert_eq!(
        db.data().await.get("key4")?.into::<Vec<i64>>()?,
        vec![3, 4, 5]
    );
    db.update().await.get("key5")?.push("!")?;
    assert_eq!(
        db.data().await.get("key5")?.into::<Vec<String>>()?,
        vec!["Welcome", "to", "NanoDB", "!"]
    );

    // remove
    db.update().await.remove("key3")?;
    assert!(db.data().await.get("key3").is_err());

    Ok(())
}
