# Impl keyword
The `Impl` (implementation) keyword is used to add methods to a struct.

## How to use
Let`s say that you have the following struct:
```rust
struct Rectangle {
  width: u32,
  height: u32
}
```
And you want to, let's say, verify if it is a <b>square</b> or not. We can implement this method like this:
```rust
struct Rectangle {
  width: u32,
  height: u32
}

impl Rectangle {
  fn is_square(&self){
    self.width == self.height
  }
}
```
And then, to use it, we can do:
```rust
fn main(){
  let my_rect = Rectangle {width: 10, height: 5};

  println!("Rectangle is a square: {}", my_rect.is_square());
}
```