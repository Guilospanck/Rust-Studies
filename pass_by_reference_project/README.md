# Pass By Reference
You pass the address of some variable to a function.
We do that because if you try the following, for example:
```rust
struct Color {
  red: u8,
  green: u8,
  blue: u8
}

fn main() {
  let blue = Color { red: 0, green: 0, blue: 255};

  print_color(blue); // ok
  print_color(blue); // ERROR: variable moved
}

fn print_color(c: Color){
  println!("R: {} G: {} B: {}", c.red, c.green, c.blue);
}
```
In order to fix the example above, you have to pass the function as reference, like this:
```rust
struct Color {
  red: u8,
  green: u8,
  blue: u8
}

fn main() {
  let blue = Color { red: 0, green: 0, blue: 255};

  print_color(&blue); // ok
  print_color(&blue); // ok
}

fn print_color(c: &Color){
  println!("R: {} G: {} B: {}", c.red, c.green, c.blue);
}
```
