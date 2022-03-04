# Running/Executing Commands (CLI)
Using CLI in Rust.

## Concepts
You can use it by adding the reference to the `Command` package:
```rust
use std::process::Command;
```
And then, if you want to open a python file, for example, called `guilherme.py`, you can do:
```rust
fn main() {
  // python guilherme.py
  let mut cmd = Command::new("python");
  cmd.argv("guilherme.py");

  // Execute the command
  match cmd.output() {
    Ok(value) => println!("{}", String::from_utf8(value.stdout).unwrap()),
    Err(_) => println!("Error executing command")
  }

}
```