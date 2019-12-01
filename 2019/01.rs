use std::fs::File;
use std::io::prelude::*;

fn get_input() -> std::io::Result<String> {
  let mut file = File::open("01.txt")?;
  let mut contents = String::new();
  file.read_to_string(&mut contents)?;

  Ok(contents)
}

fn get_fuel(mass: u64) -> u64 {
  (mass / 3).checked_sub(2).unwrap_or(0)
}

fn get_fuel_recursive(mass: u64) -> u64 {
  let initial_fuel = get_fuel(mass);

  if initial_fuel <= 0 {
    0
  } else {
    initial_fuel + get_fuel_recursive(initial_fuel)
  }
}

fn main() {
  let input = get_input().unwrap();
  let fuel = input.lines()
    .filter_map(|line| line.parse::<u64>().ok())
    .map(get_fuel)
    .sum::<u64>();

  println!("Part 1: {}", fuel);

  let fuel = input.lines()
    .filter_map(|line| line.parse::<u64>().ok())
    .map(get_fuel_recursive)
    .sum::<u64>();

  println!("Part 2: {}", fuel);
}
