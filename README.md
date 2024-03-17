# NanoDB
NanoDB is a simple, lightweight, and easy-to-use JSON database for Rust. It is designed to be used in small to medium-sized projects where a full-fledged database is not necessary. It provides a simple and easy-to-use API for reading and writing JSON data.
NanoDB provides the following features:
* **Simple API**: Easy interface for JSON data manipulation.
* **Atomic Operations**: Guarantees data consistency with atomic reads and writes.
* **Tree Structures**: Supports focused access and modifications with specialized trees.
* **Asynchronous I/O**: Enhances performance with non-blocking I/O.
* **Thread Safety**: Safe for multi-threaded use.


## Trees
NanoDB distinguishes three specialized types of trees, each designed to interact with JSON data efficiently and safely within different contexts:
* **Tree**: This structure encapsulates a cloned subtree of the original JSON data. The clone represents a specific segment of the original data, pinpointed by a designated path within the JSON structure. This enables focused access to a discrete portion of the data, facilitating operations on this subset without affecting the rest of the database's data. NanoDB allows to merge trees back into the main database if wanted.
* **ReadGuardedTree**: Building upon the basic Tree structure, a ReadGuardedTree includes a read lock mechanism. This enhancement permits concurrent read operations by multiple threads while guaranteeing that the data remains unchanged during these operations. The mechanism allows for multiple ReadGuardedTrees to exist simultaneously, provided they are only being used for reading, thus ensuring data consistency without hindering accessibility.
* **WriteGuardedTree**:  Similar to a ReadGuardedTree but with a critical difference: it features a write lock to facilitate atomic write operations. This exclusive lock ensures that only one WriteGuardedTree can perform write operations at any given time, thereby preventing concurrent modifications that could lead to data inconsistencies or race conditions. This strict control mechanism is pivotal for maintaining the integrity of the database when updates are being made.

These specialized tree structures are fundamental to NanoDB's design, enabling a balance between concurrent data access and modification safety. By delineating clear roles and access controls for each tree type, NanoDB ensures data integrity and consistency, whether in read-heavy or write-intensive scenarios.

## Examples
Check out the examples folder for more.
```rust
use nanodb::{error::NanoDBError, nanodb::NanoDB, trees::tree::Tree};
use serde::Deserialize;
use serde_json::{json, Map, Value};
//...

#[derive(Deserialize)]
pub struct MyStruct {
    name: String,
    versions: Vec<f64>,
}

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

// Basic reads (working with cloned sub-tree)
let text: String = db.data().await.get("key1")?.into()?;
let number: i64 = db.data().await.get("key2")?.into()?;
let object: MyStruct = db.data().await.get("key3")?.into()?;
let array: Vec<i64> = db.data().await.get("key4")?.into()?;

// Basic inserts
db.insert("age", 42).await?; // shorthand for db.update().await.insert("age", 42)?;
db.insert("crates", vec!["tokio", "serde"]).await?;
db.insert("some_map", Map::new()).await?;
db.insert("person", json!({"name": "Donald"})).await?;
db.write().await?; // write to file if needed

// Atomic CRUD operations and advanced data manipulation
db.update().await.remove("age")?;
db.update().await.get("key3")?.insert("key", "value")?;
db.update().await.get("key3")?.get("versions")?.push(42.0)?;
db.update().await.get("key4")?.for_each(|v| {
	*v = Value::from(v.as_i64().unwrap() * 2i64);
})?;
db.write().await?; // write to file if needed

// Atomic reader (read() returns a read lock on the db)
let versions: Vec<String> = db.read().await.get("key5")?.into()?;
let number: i64 = db.read().await.get("key4")?.at(0)?.into()?;

// Tree methods and merging
let mut my_tree: Tree = db.data().await.get("key3")?;
my_tree.insert("language", "Rust")?;
db.insert_tree(my_tree).await?;
db.write().await?;

// Advanced write locks
let mut write_lock = db.update().await;
write_lock.insert("key1", "Welcome to NanoDB")?;
write_lock.insert("key1", "Welcome to NanoDB again")?;
write_lock.release_lock(); // release the lock manually to avoid deadlocks
let mut another_write_lock = db.update().await;
assert_eq!(
	another_write_lock.get("key1")?.into::<String>().unwrap(),
	"Welcome to NanoDB again"
);
```
## Contributing

Contributions are absolutely, positively welcome and encouraged! Contributions
come in many forms. You could:

  1. Submit a feature request or bug report as an [issue].
  2. Ask for improved documentation as an [issue].
  3. Ask for help [issues asking for help].
  4. Ask any question [issues asking questions]

[issue]: https://github.com/2bonahill/nanodb/issues
[issues asking for help]: https://github.com/2bonahill/nanodb/labels/help%20wanted
[issues asking questions]: https://github.com/2bonahill/nanodb/labels/question