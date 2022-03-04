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

  // replace
  {
    let my_string: String = String::from("Rust is fantastic!");
    println!("After replace: {}", my_string.replace("fantastic", "great"));
  }
  
  // separate into lines
  {
    let my_string: String = String::from("Hello\nThis is the first line\nThis is the second line");
    for line in my_string.lines() {
      println!("[ {} ]", line);
    }
  }

  // split
  {
    let my_string: String = String::from("Leave+a+like+if+you+like+it");
    let tokens: Vec<&str> = my_string.split("+").collect();
    for token in tokens {
      print!("{}-\n", token);
    }
  }

  // strim
  {
    let my_string: String = String::from("     So much space!    ");
    println!("Trimmed:\n{}", my_string.trim());
  }

  // char
  {
    let my_string: String = String::from("Guilherme");

    match my_string.chars().nth(4) {
      Some(c) => println!("Found char at index 4: {}", c),
      None => println!("No chat at index 4.")
    }

  }

}
