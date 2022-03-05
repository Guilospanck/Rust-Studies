# Working with JSON
JSON in Rust.

## Concepts
To parse and stringify (marshall and unmarshall) a JSON in Rust you need to have the following:

In your `Cargo.toml` file, add under dependencies:
```toml
[dependencies]
serde = { version = "1.0", features = ["derive"] }
serde_json = "1.0"
```
> Note: The version of serde_json must be compatible with the version of serde.

Then, in your code, do:
```rust
// main.rs
use serde::{Serialize, Deserialize};

fn main() {

}
```

And, to actually deserialize/serialize a struct (JSON), you can do:
```rust
// main.rs
use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize, Debug)] // debug is only needed if you want to print out the whole struct
struct Point {
  x: i32,
  y: i32
}

fn main() {

  let point = Point {x: 10, y: 20};

  let serialized = serde_json::to_string(&point).unwrap();
  println!("Serialized: {}", serialized);

  let deserialized = serde_json::from_str(&point).unwrap();
  println!("Deserialized: {:?}", deserialized); // debug needed
  println!("Deserialized: {}", deserialized.x); // debug not needed

}
```