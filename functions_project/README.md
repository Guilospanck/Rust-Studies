# Functions project
Some concepts about functions in Rust.

## Concepts
- The `return` keyword can be used to return a value inside a function's body.
When this expression is not used, the last <b>expression</b> is implicitly
considered to be the return value.
Example:
```rust
func add(x: i8, y: i8) -> i8 {
  x+y // works
  x+y; // doesn't work because of the semicolon at the end, turning it into a statement and not a expression
}
```
or
```rust
func add(x: i8, y: i8) -> i8 {
  return x+y
}
```
- Statements vs Expressions:
  - statements do not return value; expressions do.
  - a semicolon at the end of an expression turns it into a statement.
  - `{}` blocks and function calls are expressions.