mod guilherme;

mod person {
  fn chicken(){
    println!("I'm a chicken!");
  }

  pub fn print_person_name(){
    println!("Hello, I'm a person called Guilherme and");
    chicken();
  }

  pub mod snake {
    pub fn print_snake_name(){
      println!("I'm a snake!");
    }
  }
}

fn main() {
  guilherme::print_name();

  person::print_person_name();
  person::snake::print_snake_name();
}
