use std::fs::File;
use std::io::prelude::*;

mod intcode;
use intcode::*;

fn get_input() -> std::io::Result<String> {
  let mut file = File::open("09.txt")?;
  let mut contents = String::new();
  file.read_to_string(&mut contents)?;

  Ok(contents)
}

fn main() {
  let input = get_input().unwrap();
  let numbers = input.split(',')
    .filter_map(|x| x.trim().parse::<i64>().ok())
    .collect::<Vec<_>>();

  let mut state = ProgramState::new(numbers.clone());
  let (outputs, _) = run_program_with_inputs(&mut state, std::iter::once(1));
  println!("Part 1: {}", outputs.last().unwrap());

  let mut state = ProgramState::new(numbers.clone());
  let (outputs, _) = run_program_with_inputs(&mut state, std::iter::once(2));
  println!("Part 2: {}", outputs.last().unwrap());
}
