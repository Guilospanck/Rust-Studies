fn main() {
  let _x: i32 = 1; // integer
  let _y: f64 = 2.01; // float
  let _name: &str = "This is a string"; // string
  let _character: char = 'c'; // char
  let _booleanvariable: bool = true; // boolean

  // x = 12; // error: cannot assign to immutable variable

  let mut a: i32 = 2; // mutable i32
  println!("The value of a before is: {}", a);  

  a = 12; // no error. this is a mutable variable
  println!("The value of a after is: {}", a);  


  let my_array: [i32; 6] = [1, 2, 3, 4, 5, 6]; // array defined size
  let _type_inference_array = [1, 2, 3, 4]; // type inference array

  println!("Array at 5: {}", my_array[5]);
  
  let my_tuple: (i32, f64, &str) = (5, 5.0, "Guilherme"); // tuple
  let (dynamic_int, dynamic_float, dynamic_string) = my_tuple; // assigning tuple to values

  println!("Dynamic int: {}, Dynamic float: {}, Dynamic string: {}", dynamic_int, dynamic_float, dynamic_string);

}
