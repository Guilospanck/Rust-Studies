use reqwest;

fn main() {
  let response_text = reqwest::get("https://jsonplaceholder.typicode.com/todos/1")
    .expect("Could not make request")
    .text()
    .expect("Could not retrieve response text");

  println!("{}", response_text);
}
