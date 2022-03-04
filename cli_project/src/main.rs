use std::process::Command;

fn main() {
  // python3 guilherme.py
  let mut cmd = Command::new("python3");
  cmd.arg("guilherme.py");

  // Execute command
  match cmd.output() {
    Ok(o) => println!("{}", String::from_utf8(o.stdout).unwrap()),
    Err(_) => println!("Error executing command!")
  }
}
