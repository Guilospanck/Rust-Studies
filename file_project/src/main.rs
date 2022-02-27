use std::fs::File;
use std::io::prelude::*;

fn main() {
  /* Read from a file */
  let mut file = File::open("info.txt").expect("Can't open file!");

  let mut contents = String::new();
  file.read_to_string(&mut contents)
      .expect("Ops! Can not read the file...");

  println!("File contents:\n{}", contents);

  /* Write to a file */
  let mut write_file = File::create("output.txt").expect("Could not create a file");
  write_file.write_all(b"Wrote to a file").expect("Could not write to the file. Sorry.");

}
