mod simple;

use nanodb::{error::NanoDBError, nanodb::NanoDB};

extern crate nanodb;

#[tokio::main]
async fn main() -> Result<(), NanoDBError> {
    // spawn a new task
    // Use `tokio::spawn` to run asynchronous tasks
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
        // handle.await.unwrap(); // `await` makes sure the program waits for the task to finish
        // wait for all the handles to finish
        handle.join().unwrap();
    }

    println!("All threads have completed.");

    // // Tokio Threads
    // let mut handles = Vec::new();
    // for i in 0..10 {
    //     let mut db_clone = db.clone();
    //     let handle = tokio::spawn(async move {
    //         let mut numbers = db_clone.get("numbers").unwrap();
    //         numbers.push(i).unwrap();
    //         db_clone.merge(numbers).unwrap();
    //         db_clone.write_async().await.unwrap();
    //     });
    //     handles.push(handle);
    // }

    // // Await all tasks to complete
    // for handle in handles {
    //     handle.await.unwrap(); // `await` makes sure the program waits for the task to finish
    // }

    Ok(())
}
