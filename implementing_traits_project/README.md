# Implementing Traits
Understanding the implementation of `traits` in Rust.

## Concepts
### Traits
Basically you can say that a trait is like an interface. Those who implement the trait, must implement its methods.

## Example
Let's say that we have a trait like this:
```rust
pub trait ToString {
  fn to_string(&self) -> String;
}
```
Those who will implement this trait, must implement the `to_string` method.

Implementation example:
```rust
struct Person {
  name String,
  age u8
}

impl ToString for Person {
  fn to_string(&self) -> String { // look how the function signature must be the same as the trait
    return format!("My name is {} and I'm {}", self.name, self.age);
  }
}

fn main(){
  let my_person = Person{name: String::from("Guilherme"), age: 26};

  println!("{}", my_person.to_string()); // My name is Guilherme and I'm 26
}
```