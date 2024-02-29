mod simple;

use std::time::Duration;

use nanodb::error::NanoDBError;
use tokio::time::sleep;
extern crate nanodb;

#[tokio::main]
async fn main() -> Result<(), NanoDBError> {
    // spawn a new task
    // Use `tokio::spawn` to run asynchronous tasks
    let mut handles = Vec::new();
    for i in 0..10 {
        let handle = tokio::spawn(async move {
            println!("Task {} is running", i);
            sleep(Duration::from_secs(1)).await; // Non-blocking sleep
            println!("Task {} has finished", i);
        });
        handles.push(handle);
    }

    // Await all tasks to complete
    for handle in handles {
        handle.await.unwrap(); // `await` makes sure the program waits for the task to finish
    }

    println!("All threads have completed.");

    Ok(())
}
