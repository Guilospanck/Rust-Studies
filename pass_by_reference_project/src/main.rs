struct Color {
  red: u8,
  green: u8,
  blue: u8,
}

fn main() {
  let mut blue = Color {
    red: 0,
    green: 0,
    blue: 255,
  };

  println!("Before:");
  print_color(&blue); // 0 0 255
  
  change_to_red(&mut blue);
  
  println!("After:");
  print_color(&blue); // 255 0 0

}

fn print_color(c: &Color) {
  println!("R: {} G: {} B: {}", c.red, c.green, c.blue);
}

fn change_to_red(c: &mut Color){
  c.blue = 0;
  c.green = 0;
  c.red = 255;
}