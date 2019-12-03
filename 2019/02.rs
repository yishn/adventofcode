use std::fs::File;
use std::io::prelude::*;

fn get_input() -> std::io::Result<String> {
  let mut file = File::open("02.txt")?;
  let mut contents = String::new();
  file.read_to_string(&mut contents)?;

  Ok(contents)
}

fn run_program(program: &mut Vec<usize>) {
  let mut pointer = 0;

  loop {
    let target_value = {
      let get = |p| program[program[p]];

      match program[pointer] {
        1 => get(pointer + 1) + get(pointer + 2),
        2 => get(pointer + 1) * get(pointer + 2),
        99 => break,
        _ => panic!()
      }
    };

    let target_index = program[pointer + 3];
    program[target_index] = target_value;

    pointer += 4;
  }
}

fn calculate_output(mut program: Vec<usize>, input: (usize, usize)) -> usize {
  program[1] = input.0;
  program[2] = input.1;

  run_program(&mut program);

  program[0]
}

fn main() {
  let input = get_input().unwrap();
  let program = input.split(',')
    .filter_map(|x| x.trim().parse::<usize>().ok())
    .collect::<Vec<_>>();

  println!("Part 1: {}", calculate_output(program.clone(), (12, 2)));

  for i in 0..=99 {
    for j in 0..=99 {
      let output = calculate_output(program.clone(), (i, j));

      if output == 19690720 {
        println!("Part 2: {}", i * 100 + j);
        return;
      }
    }
  }
}
