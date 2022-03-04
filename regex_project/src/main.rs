use regex::Regex;

fn main() {
  let re = Regex::new(r"(\w{9})").unwrap(); // gets a word of 9 letters
  let text = "Guilherme";

  let is_match: bool = re.is_match(text);
  println!("Is match? {}", is_match);

  match re.captures(text) {
    Some(caps) => println!("Found match: {}", &caps[0]),
    None => println!("Could not find a match.")
  }
}
