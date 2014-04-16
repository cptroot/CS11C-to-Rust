extern crate rand;
use std::io::{stdin, print};
use std::io::stdio::flush;
use std::io::BufferedReader;
use rand::random;

fn main() {
  print("Enter your name: ");
  flush();
  let mut reader = BufferedReader::new(stdin());
  match reader.read_line() {
    Err(_) => fail!("Recieved EOF instead of a name."),
    Ok(name) => {
      let name = name.trim_right();

      let n = random::<uint>() % 10 + 1;

      for _ in range(0, n) {
        if n % 2 == 0 {
          println!("{}: Hello, {}!", n, name);
        } else {
          println!("{}: Hi there, {}!", n, name);
        }
      }
    }
  }
}
