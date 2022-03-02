# Random numbers and booleans
Random number and booleans in Rust using external lib `rand`.

## Concepts
In order to add a foreign dependency, you need to open `Cargo.toml` and add it under dependencies
```toml
[dependencies]
rand = "*"
```
Then in your `.rs` file, you have to add the:
```rust
extern crate rand; // to use the external library
use rand::Rng; // what you want to use from that library
```