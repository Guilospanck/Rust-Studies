# Multiple sources
Using multiple sources in Rust (modules).

## Concepts
In Rust, every method/function is private by default. In order to make it public, you have to put before the statement the `pub` keyword (which means "public").

Once you have your file with the functions/method that you want public defined, you have to then call your file.

You do that by adding to the file you want to use the other functions the keywords:
```rust
mod filename;
```
So, for example you have a file called `guilherme.rs` which has the contents:
```rust
// guilherme.rs
pub fn print_name() {
  println!("Hello, my name is Guilherme");
}
```
And you want to use it in your `main.rs` file, you do:
```rust
// main.rs
mod guilherme;

fn main(){
  guilherme::print_name();
}
```