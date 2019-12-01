use std::fs::File;
use std::io::prelude::*;

fn get_input() -> std::io::Result<String> {
  let mut file = File::open("01.txt")?;
  let mut contents = String::new();
  file.read_to_string(&mut contents)?;

  Ok(contents)
}

fn get_fuel(mass: i32) -> i32 {
  *[mass / 3 - 2, 0].into_iter().max().unwrap()
}

fn get_fuel_recursive(mass: i32) -> i32 {
  let mut result = get_fuel(mass);
  let mut residual_fuel = result;

  loop {
    residual_fuel = get_fuel(residual_fuel);

    if residual_fuel <= 0 {
      break;
    }

    result += residual_fuel;
  }

  result
}

fn main() {
  let input = get_input().unwrap();
  let fuel = input.lines()
    .filter_map(|line| line.parse::<i32>().ok())
    .map(get_fuel)
    .sum::<i32>();

  println!("Part 1: {}", fuel);

  let fuel = input.lines()
    .filter_map(|line| line.parse::<i32>().ok())
    .map(get_fuel_recursive)
    .sum::<i32>();

  println!("Part 2: {}", fuel);
}
