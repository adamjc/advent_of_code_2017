use std::fs::File;
use std::io::prelude::*;

fn main () {
  let mut f = File::open("./src/input.txt").expect("file not found");

  let mut contents = String::new();
  f.read_to_string(&mut contents)
    .expect("something went wrong reading the file");

  let rows: Vec<&str> = contents.split('\n').collect();
  let mut checksum: u32 = 0;
  for row in rows {
    let numbers: Vec<&str> = row.split('\t').collect();

    let mut largest: u32 = u32::min_value();
    let mut smallest: u32 = u32::max_value();
    for number in numbers {
      let number = number.parse::<u32>();

      match number {
        Ok(n) => {
          if n > largest {
            largest = n;
          }
          if n < smallest {
            smallest = n;
          }
        },
        Err(_) => {
          largest = 0;
          smallest = 0;
        }
      }
    }

    checksum += largest - smallest;
  }

  println!("checksum is: {}", checksum);
}
