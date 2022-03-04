# Writing and Running Tests
How to write and run tests in Rust.

To run tests:
```bash
cargo test
```

## Concepts
You can create your tests by creating a module (`mod`) inside the file you are testing.
```rust
// main.rs
fn main() {

}

mod main_tests {

}
```
You can put the definition `#[cfg(test)]` before the `mod` in order to tell the compiler 
that this is a test and should not be built.
```rust
// main.rs
fn main() {

}

#[cfg(test)]
mod main_tests {
  
}
```
Each test should have the definition `#[test]` before it to tell the compiler that that function is a test.
```rust
// main.rs
fn main() {

}

#[cfg(test)]
mod main_tests {
  
  #[test]
  fn test_basic(){
    assert!(1 == 1);
  }

}
```
When you expect a test to `panic`, you can put the definition `#[should_panic]` before it:
```rust
// main.rs
fn main() {

}

#[cfg(test)]
mod main_tests {
  
  #[test]
  #[should_panic]
  fn test_basic(){
    assert!(1 == 2);
  }

}
```
You can also `ignore` a test by writing the `#[ignore]` definition before it:
```rust
// main.rs
fn main() {

}

#[cfg(test)]
mod main_tests {
  
  #[test]
  #[should_panic]
  fn test_basic(){
    assert!(1 == 2);
  }

  #[test]
  #[ignore]
  fn test_ignored() {
    assert!(2 == 2)
  }

}
```
You can call a function from outside using the `super::` prefix:
```rust
// main.rs
fn main() {

}

fn get_two() -> i32 {
  2
}

#[cfg(test)]
mod main_tests {
  
  #[test]
  #[should_panic]
  fn test_basic(){
    assert!(1 == 2);
  }

  #[test]
  #[ignore]
  fn test_ignored() {
    assert!(2 == 2)
  }

  #[test]
  fn test_calling_outside_function(){
    assert_eq!(super::get_two(), 1 + 1); // expected, got
  }

}
```
You can also test `structs`:
```rust
// main.rs

struct Rectangle {
  width: i8,
  height: i8
}

impl Rectangle {
  fn is_square(&self) -> bool {
    self.width == self.height
  }
}

fn main() {

}

fn get_two() -> i32 {
  2
}

#[cfg(test)]
mod main_tests {
  
  #[test]
  #[should_panic]
  fn test_basic(){
    assert!(1 == 2);
  }

  #[test]
  #[ignore]
  fn test_ignored() {
    assert!(2 == 2)
  }

  #[test]
  fn test_calling_outside_function(){
    assert_eq!(super::get_two(), 1 + 1); // expected, got
  }

  #[test]
  fn test_struct() {
    let rectangle = super::Rectangle {
      width: 50,
      height: 50
    }

    // assert_eq!(true, rectangle.is_square()); // this or the next
    assert!(rectangle.is_square());
  }

  #[test]
  #[should_panic]
  fn test_struct_not_square() {
    let rectangle = super::Rectangle {
      width: 50,
      height: 40
    }

    // assert_eq!(true, rectangle.is_square()); // this or the next
    assert!(rectangle.is_square());
  }
}
```