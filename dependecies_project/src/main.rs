use std::net::TcpListener;

// simple http server
fn main() {
  let listener = TcpListener::bind("127.0.0.1:8000").unwrap();

  for stream in listener.incoming() {
    let data = stream.unwrap();

    println!("A connection was made!\n {:?}", data); // {:?} is used to pretty print something
  }
}
