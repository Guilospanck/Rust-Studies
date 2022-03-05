use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
struct Point {
  x: i32,
  y: i32
}

fn main() {
  let point = Point {x: 10, y: 20};

  let serialized = serde_json::to_string(&point).unwrap();
  println!("Serialized: {}", serialized);

  let deserialized: Point = serde_json::from_str(&serialized).unwrap();
  println!("Deserialized: {:?}", deserialized);
  
}
