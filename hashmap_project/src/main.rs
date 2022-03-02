use std::collections::HashMap;

fn main() {
  let mut marks = HashMap::new();

  // add values
  marks.insert("Rust", 75);
  marks.insert("Go", 85);
  marks.insert("JavaScript", 88);
  marks.insert("C", 65);

  // length of Hashmap
  println!("Length of the hashmap is {}", marks.len());

  // get a single value
  match marks.get("Rust") {
    Some(mark) => println!("You've studied Rust and got {}%!", mark),
    None => println!("You haven't studied this subject")
  }

  // Remove a value
  marks.remove("C");
  println!("Length of the hashmap after remove is {}", marks.len());

  // loop through values
  for (subject, mark) in &marks {
    println!("For {} you got {}%!", subject, mark);
  }
  
  // Check for value
  println!("Did you study C++? {}", marks.contains_key("C++"));
}
