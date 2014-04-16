use std::io::{stdin, print, println};
use std::io::stdio::flush;
use std::io::BufferedReader;

fn print_spaces(num:int) {
  for _ in range(0, num) {
    print(" ");
  }
}

pub fn triangle_print(board:&[bool]) {
  for line in range(0, 5) {
    let prefix_spaces = (4 - line) * 4;
    let start = (line * (line + 1)) / 2;

    print_spaces(prefix_spaces);

    for index in range(start, line + start + 1) {
      match board[index] {
        false => print("    "),
        true  => print(" __ ")
      };
      print("    ");
    }
    println("");

    print_spaces(prefix_spaces);

    for index in range(start, line + start + 1) {
      match board[index] {
        false => print!(" {:2d} ", index),
        true  => print!("|{:2d}|", index)
      };
      print("    ");
    }
    println("");

    print_spaces(prefix_spaces);

    for index in range(start, line + start + 1) {
      match board[index] {
        false => print("    "),
        true  => print("|__|")
      };
      print("    ");
    }
    println("");
  }
}

fn get_pegnum<T: Reader>(prompt:&str, reader:&mut BufferedReader<T>) -> Option<int> {
  loop {
    print(prompt);
    flush();
    match reader.read_line().ok().unwrap().trim() {
      "" => { continue; },
      "end" => { return None },
      num => {
        let num = from_str::<int>(num).unwrap();
        if num < 0 || num > 14 {
          println("You must enter either a peg number (0 to 14, inclusive)");
          println("or the word \" end\". (without the quotes)");
          continue;
        }
        return Some(num);
      }
    }
  }
}

pub fn triangle_input(board:&mut [bool]) {
  for index in range(0, 15) {
    board[index] = false;
  }

  let mut reader = ~stdin();

  loop {
    triangle_print(board);
    println("");
    println("Enter a number to insert/remove a peg from that space, or enter the");
    println("word \"end\" when the board is set up.");
    println("");

    match get_pegnum("Input: ", reader) {
      Some(peg_num) => {
        board[peg_num] = !board[peg_num];
      },
      None => {return;}
    }
  }
}
