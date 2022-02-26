fn main() {
  let x = 10;
  println!("{}", x);

  // shadowing the previous binding
  let x = "now is a string";
  println!("{}", x);

  // shadowing the previous binding
  let x = false;
  println!("{}", x);
}
