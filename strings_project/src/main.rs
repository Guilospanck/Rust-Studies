fn main() {
  // let mut my_string = String::from("Hello, I'm a string");
  let mut my_string = "Hello, I'm a string";

  // length
  println!("Length: {}", my_string.len());

  // is Empty? boolean
  println!("Is string empty? {}", my_string.is_empty());

  // split whitespaces
  for token in my_string.split_whitespace() {
    println!("{}", token);
  }

  // contains
  println!(
    "Does the string contains `Hello`? {}",
    my_string.contains("Hello")
  );

  // push to str
  // only works with String::from
  // my_string.push_str(". Nice to meet you!");

  // working with string the most basic way:
  let str_to_add = ". Nice to Meet you!";
  let result = format!("{}{}", my_string, str_to_add);
  my_string = &result;

  println!("{}", my_string);
}
