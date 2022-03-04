#![allow(dead_code)] // will allow variables that we aren't using

enum Direction {
  Up,
  Down,
  Left,
  Right,
}

enum Day {
  Monday, Tuesday, Wednesday, Thursday, Friday,
  Saturday, Sunday
}

impl Day {
  fn is_week_day(&self) -> bool {
    match self {
      Day::Saturday | Day::Sunday => false,
      _ => true
    }
  }
}

fn main() {
  let player_direction: Direction = Direction::Up;

  // like a switch-case
  match player_direction {
    Direction::Up => println!("Heading Up direction"),
    Direction::Down => println!("Heading up direction"),
    Direction::Left => println!("Heading Left direction"),
    Direction::Right => println!("Heading Right direction"),
  }

  let day = Day::Friday;
  println!("Is today a week day? {}", day.is_week_day());
}
