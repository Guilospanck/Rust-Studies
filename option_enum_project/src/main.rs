fn main() {
  println!("The occupation is {}!", match get_occupation("Guilherme") {
    Some(o) => o,
    None => "no occupation found"
  });
}

fn get_occupation(name: &str) -> Option<&str> {
  match name {
    "Guilherme" => Some("Software Developer"),
    "Sarah" => Some("Lawyer"),
    _ => None
  }
}