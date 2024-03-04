mod sync;

use nanodb::{error::NanoDBError, nanodb::NanoDB};

extern crate nanodb;

#[tokio::main]
async fn main() -> Result<(), NanoDBError> {
    // spawn a new task
    // Use `tokio::spawn` to run asynchronous tasks

    Ok(())
}
