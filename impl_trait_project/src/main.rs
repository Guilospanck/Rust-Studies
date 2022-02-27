struct Person {
  name: String,
  age: i8,
}

trait HasVoiceBox {
  fn speak(&self);
  fn can_speak(&self) -> bool;
}

impl HasVoiceBox for Person {
  fn speak(&self) {
    println!("Hello. My name is {} and I'm {}.", self.name, self.age);
  }

  fn can_speak(&self) -> bool {
    if self.age > 0 {
      return true;
    }
    return false;
  }
}

fn main() {
  let new_person = Person {
    name: String::from("Guilherme"),
    age: 25,
  };

  println!("Can {} speak? {}", new_person.age, new_person.can_speak());

  if new_person.can_speak() {
    new_person.speak()
  }
}
