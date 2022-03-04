struct Rectangle {
  width: i8,
  height: i8
}

impl Rectangle {
  fn is_square(&self) -> bool {
    self.width == self.height
  }
}

fn main() {
  println!("Hello, world!");
}

fn get_two() -> i32 {
  2
}

#[cfg(test)]
mod main_tests {

  #[test]
  fn test_basic(){
    assert!(2 == 2);
  }

  #[test]
  fn test_outside_fn(){
    assert_eq!(2, super::get_two());
  }

  #[test]
  #[should_panic]
  fn test_panic_outside_fn(){
    assert_eq!(3, super::get_two());
  }

  #[test]
  fn test_struct(){
    let r = super::Rectangle{
      width: 50,
      height: 50
    };

    assert!(r.is_square());
  }

  #[test]
  #[should_panic]
  fn test_struct_not_square(){
    let r = super::Rectangle{
      width: 50,
      height: 40
    };

    assert!(r.is_square());
  }


}
