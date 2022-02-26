struct Color {
  red: u8,
  green: u8,
  blue: u8,
}

// tuple struct
struct ColorTuple(u8, u8, u8);

fn main() {
  let mut bg = Color {
    red: 255,
    green: 70,
    blue: 15,
  };

  bg.blue = 35;

  println!("{}, {}, {}", bg.red, bg.green, bg.blue);

  let mut bg_tuple_red = ColorTuple(255, 0, 60);
  bg_tuple_red.2 = 0;
  println!("Red is {}, {}, {}", bg_tuple_red.0, bg_tuple_red.1, bg_tuple_red.2);
}
