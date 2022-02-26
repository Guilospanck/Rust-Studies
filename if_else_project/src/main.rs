fn main() {
  let number: i32 = 5;
  let result: bool = is_number_greater_than_10(number);

  if result {
    println!("Yes, {} is greater than 10!", number);
    return;
  }

  println!("No, {} is not greater than 10!", number);
}

fn is_number_greater_than_10(number: i32) -> bool {
  if number > 10 {
    return true;
  }
  return false;
}
