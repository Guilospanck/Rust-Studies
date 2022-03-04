# HTTP Get Request with Reqwest
We can make http requests in Rust using the dependency `reqwest`.

## Concepts
Add it to the `toml` file:
```toml
[dependencies]
reqwest = "*"
```
And then use it like:
```rust
// main.rs
use reqwest;

fn main() {
  let response_text = reqwest::get("url")
    .expect("Could not make the request")
    .text().expect("Could not get response text");

  println!("Response text: {}", response_text);
}
```

### OBS
If you get an OpenSsl error while compiling change reqwest version to 0.9 and install `sudo apt install pkg-config libssl-dev`.