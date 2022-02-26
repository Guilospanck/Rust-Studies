# Variables Types Project
Understanding some variables and types concepts in Rust.

## Some general concepts
- Rust is statically typed, which means each variable can have only one type. That can't change during the runtime.
- Rust uses snake_case pattern.
- Also, it has type inference. That lets you do something like:
```rust
let x = 1;
```
  Instead of (recommended)
```rust
let x: int32 = 1;
```
- Character (char) and string are different types in Rust. Strings require double quotes("")


## Variables
Variables in Rust are supposed to be <b>immutable</b>. That means that once you declared it, you can't change it.
If you want some variable to have the possibility of changing (to be `mutable`), declare it as:
```rust
let mut a = 5;
```