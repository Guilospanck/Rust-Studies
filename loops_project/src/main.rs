fn main() {
  /* Loop */
  // infinite_loop();
  // break_loop_at_10();
  // skip_number_7();

  /* While loops */
  // while_loop();

  /* For loops */
  // for_loop();
  // for_loop_array();
  for_loop_array_with_indexes();

}

fn _infinite_loop() {
  let mut n = 0;

  loop {
    n += 1;
    println!("The value of n is {}.", n);
  }
}

fn _break_loop_at_10() {
  let mut n = 0;

  loop {
    n += 1;

    if n > 10 {
      break;
    }

    println!("The value of n is {}.", n);
  }
}

fn _skip_number_7() {
  let mut n = 0;

  loop {
    n += 1;

    if n == 7 {
      continue;
    }

    if n > 10 {
      break;
    }

    println!("The value of n is {}.", n);
  }
}

fn _while_loop() {
  let mut n = 0;

  while n < 20 {
    n += 1;
    println!("The value of n is {}.", n);
  }
}

fn _for_loop(){  
  let range = 0..10;

  for i in range {
    println!("The value of i is {}.", i);
  }
}

fn _for_loop_array(){
  let animals = ["Rabbit", "Dog", "Cat"];
  // or
  // let animals = vec!["Rabbit", "Dog", "Cat"];

  for i in animals.iter() {
    println!("The value of i is {}.", i);
  }

  println!("{:?}", animals);
}

fn for_loop_array_with_indexes(){
  let animals = vec!["Rabbit", "Dog", "Cat"];

  for (index, animal) in animals.iter().enumerate() {
    println!("The animal at {} is {}.", index, animal);
  }
}