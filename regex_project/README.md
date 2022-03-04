# Regex
Using Regex in Rust.

## Concepts
First you have to add the `regex` dependency under `[dependencies]` of `Cargo.toml`.
```toml
[dependencies]
regex = "*"
```
Now use the dependency:
```rust
use regex::Regex;
```
And then, to create a new Regex expression:
```rust
let re = Regex::new(r"(\w{9})").unwrap()
let text = "Guilherme";

let is_match = re.is_match(text);

match re.captures(text) {
  Some(caps) => println!("Found match: {}", &caps[0]),
  None => println!("Could not find match")
}
```