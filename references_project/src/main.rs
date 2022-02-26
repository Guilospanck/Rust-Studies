fn main() {
  let mut x = 10;

  {
    let x_reference = &mut x;
    *x_reference += 1;

    println!("{}", x_reference); // 11
  }

  println!("{}", x); // 11
}
