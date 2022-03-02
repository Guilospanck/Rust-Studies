use rand::Rng;

fn main() {
  let mut rng = rand::thread_rng();

  let random_number: u8 = rng.gen_range(1..21); // 1 - 20 inclusive or 1..=20
  println!("Random number: {}", random_number);

  let random_boolean: bool = rng.gen_bool(1.0/20.0); // 1 in 20 chance of being true
  println!("Random boolean: {}", random_boolean);

}
