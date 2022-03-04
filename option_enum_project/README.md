# Option (enum)
How to create option (enum) in Rust.

## Concepts
Basically a `option` returns a `Some(c)` (if it finds it) or a `None` (if can't find it).

You can create a `option` like:
```rust
// main.rs
fn main() {
  println!("The occupation is {}", match get_occupation("Guilherme") {
    Some(o) => c,
    None => "no occupation found!"
  });
}

fn get_occupation(name: &str) -> Option<&str> { // if Some returns, it will be of type &str
  match name {
    "Guilherme" => Some("Software Developer"),
    "Sarah" => Some("Lawyer"),
    _ => None
  }
}
```