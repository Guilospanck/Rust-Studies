fn main() {
  hello_world("Guilherme");
  
  let add: i8 = add(10, 20);
  println!("Add is {}", add);
}

fn hello_world(name: &str) {
  println!("Hello, {}!", name);
}

// without the return keyword
// fn add(x: i8, y: i8) -> i8 {
//   x+y
// }

// with the return keyword
fn add(x: i8, y: i8) -> i8 {
  return x+y
}