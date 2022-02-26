struct Person {
  name: String,
  age: u8
}

impl ToString for Person {
  fn to_string(&self) -> String {
    return format!("My name is {} and I'm {}", self.name, self.age);
  }
}

fn main() {
  let my_person = Person{
    name: String::from("Guilherme"),
    age: 26
  };

  println!("{}", my_person.to_string());
}
