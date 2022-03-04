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

### Defining modules (mod)
You can also create modules by using the keyword `mod`:
```rust
// main.rs
mod person {
  pub fn print_person_name(){
    println!("My name is Guilherme");
  }  
}

fn main() {
  person::print_person_name(); // My name is Guilherme
}
```
You can also call private functions from public functions:
```rust
// main.rs
mod person {
  fn chicken(){
    println!("Chicken!");
  }

  pub fn print_person_name(){
    println!("My name is Guilherme");
    chicken();
  }  
}

fn main() {
  person::print_person_name(); // My name is Guilherme \n Chicken!
}
```
And, you can also create modules inside modules:
```rust
// main.rs
mod person {
  fn chicken(){
    println!("Chicken!");
  }

  pub fn print_person_name(){
    println!("My name is Guilherme");
    chicken();
  } 

  pub mod snake {
    pub fn print_snake_name(){
      println!("Snake name!");
    }
  }
}

fn main() {
  person::print_person_name(); // My name is Guilherme \n Chicken!

  person::snake::print_snake_name(); // Snake name!
}
```
