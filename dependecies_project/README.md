# Dependencies project
Learning how to add dependencies to our project.

## Concepts
- You can add dependencies inside the `Cargo.toml` file, under `[dependencies]`, like:
```toml
[dependencies]
piston_window = "*"
```
You can declare which version or use `*` for the most recent version.
- After that, when you run `cargo run` it will automatically install required dependencies.
- Rust already comes with standard libraries, you can use them by writing at the top of files:
```rust
use std::library_needed;
```
`std` stands for "Standard".