use std::os::args;

fn main() {
  let mut is_data = false;
  let mut data = ::std::slice::with_capacity::<int>(32);

  let mut bubble_sort_flag = false;
  let mut quiet = false;

  let args = args();
  let mut args_iter = args.iter();
  args_iter.next();

  for arg in args_iter {
    match arg.as_slice() {
      "-q" => { quiet = true; },
      "-b" => { bubble_sort_flag = true; },
      arg => {
        match from_str::<int>(arg) {
          None => fail!("Invalid argument: {}", arg),
          Some(x) => {
            if data.len() == 32 {
              fail!("Given too many arguments");
            }

            data.push(x);
            is_data = true;
          }
        }
      }
    }
  }

  if !is_data {
      println!(
        "usage: {} [-b] [-q] number1 [number2 ... ] (maximum 32 numbers)",
        args[0]);
      return;
  }

  if bubble_sort_flag {
    bubble_sort(data);
  } else {
    insertion_sort(data);
  }

  if !quiet {
    for datum in data.iter() {
      println!("{}", *datum);
    }
  }
}

fn bubble_sort<T:TotalOrd>(data: &mut [T]) {
  let mut swapped = true;
  let mut end_index = data.len();

  while swapped {
    swapped = false;
    for i in range(0, end_index - 1) {
      match data[i].cmp(&data[i + 1]) {
        Greater => {
          data.swap(i, i + 1);

          swapped = true;
        },
        _ => {}
      }
    }

    end_index -= 1;
  }
}

fn insertion_sort<T:TotalOrd>(data:&mut [T]) {
  let mut min_index = 0;
  let length = data.len();
  if length <= 1 {return;}

  for i in range(0, length) {
    match data[i].cmp(&data[min_index]) {
      Less => {min_index = i;},
      _ => {}
    }
  }

  data.swap(0, min_index);

  insertion_sort(data.mut_slice(1, length));
}
