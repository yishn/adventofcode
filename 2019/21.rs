use std::fs::File;
use std::io::prelude::*;
use std::iter;

mod intcode;
use intcode::*;

#[derive(Debug, Clone)]
enum DroidResult {
  Fail(String),
  Success(i64)
}

fn get_input() -> std::io::Result<String> {
  let mut file = File::open("21.txt")?;
  let mut contents = String::new();
  file.read_to_string(&mut contents)?;

  Ok(contents)
}

fn run_springdroid(state: &mut ProgramState, script: &str) -> DroidResult {
  let inputs = script.lines()
    .map(|line| line.trim())
    .filter(|line| line.len() > 0 && line.chars().next().unwrap() != '#')
    .flat_map(|instruction| instruction.chars().chain(iter::once('\n')))
    .map(|c| c as i64);

  let (outputs, _) = run_program_with_inputs(state, inputs);

  match outputs.last() {
    Some(&x) if x > u8::max_value() as i64 => {
      DroidResult::Success(x)
    },
    _ => {
      DroidResult::Fail(
        outputs.iter()
        .map(|&c| c as u8 as char)
        .fold(String::new(), |mut acc, c| {
          acc.push(c);
          acc
        })
      )
    }
  }
}

fn main() {
  let input = get_input().unwrap();
  let program = input.split(',')
    .filter_map(|x| x.trim().parse::<i64>().ok())
    .collect::<Vec<_>>();

  let springscript = "
    # There's a hole in ABC
    NOT A J
    NOT J J
    AND B J
    AND C J
    NOT J J

    # and D is not a hole
    AND D J

    WALK
  ";

  let result = run_springdroid(&mut ProgramState::new(program.clone()), springscript);

  match result {
    DroidResult::Success(x) => println!("Part 1: {}", x),
    DroidResult::Fail(msg) => println!("Failure: {}", msg)
  }

  let springscript = "
    # (
      # E and I are not holes
      OR E J
      AND I J
    # )

    # (
      # E and F are not holes
      OR E T
      AND F T
    # )

    OR T J
    OR H J

    # (
      # There's a hole in ABC
      NOT A T
      NOT T T
      AND B T
      AND C T
      NOT T T

      # and D is not a hole
      AND D T
    # )

    AND T J

    RUN
  ";

  let result = run_springdroid(&mut ProgramState::new(program.clone()), springscript);

  match result {
    DroidResult::Success(x) => println!("Part 2: {}", x),
    DroidResult::Fail(msg) => println!("Failure: {}", msg)
  }
}
