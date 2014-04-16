extern crate rand;

use rand::random;
use std::slice;
use std::os::args;

use std::io::{print, println, stderr};

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


fn iterate_state(current_state:&[bool], next_state:&mut [bool]) {
  let mut prev = current_state.iter().peekable();
  let mut curr = current_state.iter();
  let mut next = current_state.iter().peekable();
  
  let mut curr_lvalue = next_state.mut_iter();
  
  curr.next();
  next.next();
  next.next();
  
  match curr_lvalue.next() {
    None => { return; },
    Some(cell) => { *cell = false; }
  }
  
  while !next.peek().is_none() {
    *curr_lvalue.next().unwrap() = 
      if *curr.next().unwrap() {
        false
      } else {
        *prev.peek().unwrap() != *next.peek().unwrap()
      };
    prev.next();
    next.next();
  }
  
  if !curr.next().is_none() {
    *curr_lvalue.next().unwrap() = false;
  }
}

fn print_state(current_state:&[bool]) {
  for &cell in current_state.iter() {
    if cell {
      print("*");
    } else {
      print(".");
    }
  }
  println("");
}

fn print_usage(file_name:&str) -> std::io::IoResult<()> {
  writeln!(&mut stderr() as &mut std::io::Writer,
           "usage: {} size iterations", file_name)
}
