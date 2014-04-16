use std::io::{stdin, print};
use std::io::stdio::flush;
use std::io::BufferedReader;

fn main() {
  print("Enter your name: ");
  flush();
  let mut reader = BufferedReader::new(stdin());
  match reader.read_line() {
    Err(_) => fail!("Error reading input"),
    Ok(name) => {
      let name = name.trim_right();
      println!("Hello {}!", name);
    }
  }
}
