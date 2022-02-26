# Shadowing
Shadowing consists of changing the value or type of a variable in Rust.
Basically, let's have the following:
```rust
let x = 10;
```
The shadowing consists of defining the same variable again with different values e/or types:
```rust
let mut x = 10;
println!("{}", x); // 10

let x = "now x is a string";
println!("{}", x); // "now x is a string"

let x = false;
println!("{}", x); // false
```