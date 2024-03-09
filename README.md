# NanoDB
NanoDB is a simple, lightweight, and easy-to-use JSON database for Rust. It is designed to be used in small to medium-sized projects where a full-fledged database is not necessary. It is built on top of the serde_json crate and provides a simple and easy-to-use API for reading and writing JSON data.
NanoDB provides the following features:
* **Simple API**: NanoDB provides a simple and easy-to-use API for reading and writing JSON data.
* **Atomic Reads and Writes**: NanoDB provides atomic reads and writes, ensuring that the data is always consistent.
* **Tree Methods**: NanoDB provides tree methods for working with JSON data, such as getting, setting, and merging data.
* **Asynchronous I/O**: NanoDB uses asynchronous I/O to ensure that it is fast and efficient.
* **Lightweight**: NanoDB is lightweight and has minimal dependencies, making it easy to use in small to medium-sized projects.
* **JSON Serialization and Deserialization**: NanoDB uses the serde_json crate for JSON serialization and deserialization, ensuring that it is fast and efficient.
* **Thread Safety**: NanoDB is thread-safe, ensuring that it can be used in multi-threaded applications.


## Trees
NanoDB distinguishes three specialized types of trees, each designed to interact with JSON data efficiently and safely within different contexts:
* **Tree**: This structure encapsulates a cloned subtree of the original JSON data, allowing read-only access. The clone represents a specific segment of the original data, pinpointed by a designated path within the JSON structure. This enables focused access to a discrete portion of the data, facilitating operations on this subset without affecting the rest of the database's data.
* **ReadGuardedTree**: Building upon the basic Tree structure, a ReadGuardedTree includes a read lock mechanism. This enhancement permits concurrent read operations by multiple threads while guaranteeing that the data remains unchanged during these operations. The mechanism allows for multiple ReadGuardedTrees to exist simultaneously, provided they are only being used for reading, thus ensuring data consistency without hindering accessibility.
* **WriteGuardedTree**:  Similar to a ReadGuardedTree but with a critical difference: it features a write lock to facilitate atomic write operations. This exclusive lock ensures that only one WriteGuardedTree can perform write operations at any given time, thereby preventing concurrent modifications that could lead to data inconsistencies or race conditions. This strict control mechanism is pivotal for maintaining the integrity of the database when updates are being made.

These specialized tree structures are fundamental to NanoDB's design, enabling a balance between concurrent data access and modification safety. By delineating clear roles and access controls for each tree type, NanoDB ensures data integrity and consistency, whether in read-heavy or write-intensive scenarios.

## Examples
```rust
use nanodb::{error::NanoDBError, nanodb::NanoDB, trees::tree::Tree};
use serde_json::{Map, Value};

//...

// Open the DB
let mut db = NanoDB::open("examples/data/data.json")?;

// Simple reading (working with a cloned sub-tree)
let age: i64 = db.data().await.get("age")?.into()?;
let city: String = db.data().await.get("address")?.get("city")?.into()?;
let fruit: String = db.data().await.get("fruits")?.at(1)?.into()?;
let address: Map<String, Value> = db.data().await.get("address")?.into()?;

// Simple inserts
db.insert("age", 42).await?;
db.insert("fruits", vec!["apple", "banana"]).await?;

// Atomic CRUD operations and advanced data manipulation
db.update().await.remove("age")?;
db.update().await.get("address")?.insert("key", "value")?;
db.update().await.get("hobbies")?.push("reading")?;
db.update().await.get("numbers")?.for_each(|v| {
	*v = Value::from(v.as_i64().unwrap() + 2i64);
})?;

// Write changes to the disk
db.write().await?;

// Atomic reader
let fruits: Vec<String> = db.read().await.get("fruits")?.into()?;
let first_fruit: String = db.read().await.get("fruits")?.at(0)?.into()?;

// Tree methods and merging
let mut address_tree: Tree = db.data().await.get("address")?;
address_tree.insert("zip", "12345")?;
db.insert_tree(address_tree).await?;
let numbers = db
	.data()
	.await
	.get("numbers")?
	.for_each(|v| {
		*v = Value::from(v.as_i64().unwrap() + 2i64);
	})
	.unwrap();
db.insert_tree(numbers).await?;
db.write().await?;
```
