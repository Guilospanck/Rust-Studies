struct Circle {
  radius: f64,
}

impl Circle {
  fn calculate_area(&self) -> f64 {
    let area: f64 = std::f64::consts::PI * self.radius.powi(2);
    return area;
  }

  fn get_radius(&self) -> f64 {
    self.radius
  }
}

fn main() {
  let my_circle = Circle { radius: 5.0 };

  println!("Radius of circle is {}", my_circle.get_radius());

  let area: f64 = my_circle.calculate_area();

  println!("Area of circle is {}", area);
}
