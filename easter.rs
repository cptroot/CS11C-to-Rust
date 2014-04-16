use std::io::stdin;
use std::io::stderr;
use std::io::BufferedReader;

fn main() {
  let mut reader = BufferedReader::new(stdin());

  for year in reader.lines() {
    let year = from_str::<int>(year.ok().unwrap().trim_right()).unwrap();
    let date = calculate_Easter_date(year);
    match date {
      0 => {},
      date if date > 0 => println!("{} - April {}", year, date),
      _                => println!("{} - March {}", year, -date)
    }
  }
}

fn calculate_Easter_date(year:int) -> int {
  if year < 1582 || year > 39999 {
    (writeln!(&mut stderr() as &mut std::io::Writer, "Invalid Year: {}", year))
      .ok().unwrap();
    return 0;
  }

  // STEP E1: Set G to (Y mod 19) + 1.
  //  [G is the "golden year" in the 19-year Metonic cycle.]
  let golden_year = year % 19 + 1;
  // STEP E2: Set C to (Y / 100) + 1. [C is the century]
  let century = year / 100 + 1;
  // STEP E3: Set X to (3C / 4) - 12. [X is the skipped leap years.]
  let skipped_leap_years = 3 * century / 4 - 12;
  // Set Z to ((8C + 5) / 25) - 5.
  //  [Z is a correction factor for the moon's orbit.]
  let correction_factor = (8 * century + 5) / 25 - 5;
  // STEP E4: Set D to (5Y / 4) - X - 10.
  //  [March ((-D) mod 7 + 7) is a Sunday.]
  let d = 5 * year / 4 - skipped_leap_years - 10;
  // STEP E5: Set E to (11G + 20 + Z - X) mod 30.
  //  If E is 25 and G is greater than 11 or if E is 24,
  // increment E.
  //  [E is the "epact" which specifies when a full moon occurs.]
  let mut epact = (11 * golden_year + 20 +
                   correction_factor - skipped_leap_years) % 30;
  if (epact == 25 && golden_year > 11) || epact == 24 {
    epact += 1;
  }
  // STEP E6: Set N to 44 - E.  [March N is a "calendar full moon".]
  let mut calendar_full_moon = 44 - epact;
  // If N is less than 21 then add 30 to N.
  if calendar_full_moon < 21 {
    calendar_full_moon += 30;
  }
  // STEP E7: Set N to N + 7 - ((D + N) mod 7).
  //  [N is a Sunday after full moon.]
  let sunday = calendar_full_moon + 7 - (d + calendar_full_moon) % 7;
  // STEP E8: If N > 31 the date is APRIL (N - 31),
  // otherwise the date is MARCH N.
  if sunday > 31 {
    sunday - 31
  } else {
    -sunday
  }
}
