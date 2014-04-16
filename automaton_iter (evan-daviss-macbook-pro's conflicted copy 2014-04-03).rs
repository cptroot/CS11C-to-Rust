extern crate rand;

use rand::random;
use std::slice;
use std::os::args;

use std::io::{print, println};

use std::slice::MutableCloneableVector;

fn main() {
  // get arguments

  let args = args();
  let mut args_iter = args.iter();
  args_iter.next();

  let size = from_str::<uint>(*args_iter.next().unwrap()).unwrap();
  let num_iterations = from_str::<uint>(*args_iter.next().unwrap()).unwrap();

  // Initialize arrays
  let mut current_state =
      slice::build(Some(size),
                  |push: |v: bool|| {
                  for _ in range(0, size) {
                    push(random::<bool>());
                  }});

  let mut next_state =
      slice::build(Some(size),
                   |push: |v: bool|| {
                   for _ in range(0, size) {
                     push(false); }});

  // Simulate
  print_state(current_state);
  for _ in range(0, num_iterations) {
    iterate_state(current_state, next_state);
    assert!(current_state.copy_from(next_state) == next_state.len());

    print_state(current_state);
  }
}


fn iterate_state(current_state:&mut [bool], next_state:&mut [bool]) {
  next_state[0] = false;
  for index in range(1, current_state.len() - 1) {
    if current_state[index] {
      next_state[index] = false;
    } else {
      next_state[index] = current_state[index - 1] != current_state[index + 1];
    }
  }
  next_state[current_state.len() - 1] = false;
}

fn print_state(current_state:&[bool]) {
  for index in range(0, current_state.len()) {
    if current_state[index] {
      print("*");
    } else {
      print(".");
    }
  }
    println("");
}

fn print_usage(file_name:&str) {
  writeln!(stderr(),
           "usage: {} size iterations", file_name);
}
