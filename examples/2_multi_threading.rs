use nanodb::{error::NanoDBError, nanodb::NanoDB};

extern crate nanodb;

#[tokio::main]
async fn main() -> Result<(), NanoDBError> {
    let mut db = NanoDB::open("examples/data/data.json")?;
    db.insert("counter", 0).await?;

    let mut handles = Vec::new();
    for _ in 0..10 {
        let db_clone = db.clone();
        let handle = tokio::spawn(async move {
            let mut writer = db_clone.update().await;
            let current_counter: i64 = writer
                .tree()
                .clone()
                .get("counter")
                .unwrap()
                .into()
                .unwrap();

            writer.insert("counter", current_counter + 1).unwrap();
        });
        handles.push(handle);
    }

    // Await all tasks to complete

    for handle in handles {
        handle.await.unwrap(); // `await` makes sure the program waits for the task to finish
    }

    db.write().await?;

    Ok(())
}
