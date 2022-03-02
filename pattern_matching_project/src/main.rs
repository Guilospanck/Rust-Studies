fn main() {
  let number = 41;

  match number {
    1 => println!("It's one!"),
    2 => println!("It's two!"),
    3..=20 => println!("It's between 3 and 20!"),
    21 | 22 => println!("It's either 21 or 22"),
    _ => println!("Don't know which one is")
  }

  let name = "Guilherme";

  match name {
    "Guilherme" => println!("Hey, mate. It's me!"),
    "Larry" => println!("Hello, Larry!"),
    _ => println!("Who are you???")
  }
}
