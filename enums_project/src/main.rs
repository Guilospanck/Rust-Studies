enum Direction {
  Up,
  Down,
  Left,
  Right,
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
}
