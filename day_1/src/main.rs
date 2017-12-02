use std::fs::File;
use std::io::prelude::*;

fn main () {
  let mut f = File::open("./src/input.txt").expect("file not found");

  let mut contents = String::new();
  f.read_to_string(&mut contents)
    .expect("something went wrong reading the file");
  let contents = contents.trim();

  let mut sum: u32 = 0;
  for (index, current_digit) in contents.chars().enumerate() {
    match current_digit.to_digit(10) {
      Some(current_digit) => {
        let next_digit = contents.chars().nth((index + 1) % contents.len()).unwrap();
        match next_digit.to_digit(10) {
          Some(next_digit) => {
            if current_digit == next_digit {
              sum += current_digit;
            }
          }
          None => { continue; }
        }
      },
      None => { continue; }
    }
  }

  println!("sum is: {}", sum);
}
