use std::fs::File;
use std::io::prelude::*;

#[derive(Debug)]
enum ParameterMode {
  Position,
  Immediate
}

#[derive(Debug)]
enum OperationType {
  Add,
  Multiply,
  Input,
  Output,
  JumpIfTrue,
  JumpIfFalse,
  LessThan,
  Equals,
  Halt
}

#[derive(Debug)]
struct Instruction {
  operation: OperationType,
  inputs: Vec<(ParameterMode, i32)>
}

fn get_input() -> std::io::Result<String> {
  let mut file = File::open("05.txt")?;
  let mut contents = String::new();
  file.read_to_string(&mut contents)?;

  Ok(contents)
}

fn parse_instruction(numbers: &[i32]) -> Instruction {
  let instruction_code = numbers[0];
  let op_code = instruction_code % 100;

  let (operation, inputs_count) = match op_code {
    1 => (OperationType::Add, 3),
    2 => (OperationType::Multiply, 3),
    3 => (OperationType::Input, 1),
    4 => (OperationType::Output, 1),
    5 => (OperationType::JumpIfTrue, 2),
    6 => (OperationType::JumpIfFalse, 2),
    7 => (OperationType::LessThan, 3),
    8 => (OperationType::Equals, 3),
    99 => (OperationType::Halt, 0),
    _ => panic!()
  };

  let get_parameter_mode = |i| {
    match (instruction_code - op_code as i32) / 10i32.pow(2 + i as u32) % 10 {
      0 => ParameterMode::Position,
      1 => ParameterMode::Immediate,
      _ => panic!()
    }
  };

  Instruction {
    operation,
    inputs: (0..inputs_count)
      .map(|i| (get_parameter_mode(i), numbers[i + 1]))
      .collect()
  }
}

fn run_program(program: &mut Vec<i32>, input: i32) -> Vec<i32> {
  let mut result = vec![];
  let mut pointer = 0;

  while pointer < program.len() {
    let init_pointer = pointer;
    let instruction = parse_instruction(&program[pointer..]);

    let (target_value, target_index) = {
      let get_input = |i| match instruction.inputs[i] {
        (ParameterMode::Position, j) => program[j as usize],
        (ParameterMode::Immediate, value) => value
      };

      match instruction.operation {
        OperationType::Add => (
          Some(get_input(0) + get_input(1)),
          Some(instruction.inputs[2].1 as usize)
        ),
        OperationType::Multiply => (
          Some(get_input(0) * get_input(1)),
          Some(instruction.inputs[2].1 as usize)
        ),
        OperationType::Input => (
          Some(input),
          Some(instruction.inputs[0].1 as usize)
        ),
        OperationType::Output => (
          Some(get_input(0)),
          None
        ),
        OperationType::JumpIfTrue => {
          if get_input(0) != 0 { pointer = get_input(1) as usize; }
          (None, None)
        },
        OperationType::JumpIfFalse => {
          if get_input(0) == 0 { pointer = get_input(1) as usize; }
          (None, None)
        }
        OperationType::LessThan => (
          Some(if get_input(0) < get_input(1) { 1 } else { 0 }),
          Some(instruction.inputs[2].1 as usize)
        ),
        OperationType::Equals => (
          Some(if get_input(0) == get_input(1) { 1 } else { 0 }),
          Some(instruction.inputs[2].1 as usize)
        ),
        OperationType::Halt => break
      }
    };

    if let Some(target_value) = target_value {
      if let Some(target_index) = target_index {
        program[target_index] = target_value;
      } else {
        result.push(target_value);
      }
    }

    if pointer == init_pointer {
      pointer += instruction.inputs.len() + 1;
    }
  }

  result
}

fn main() {
  let input = get_input().unwrap();
  let numbers = input.split(',')
    .filter_map(|x| x.trim().parse::<i32>().ok())
    .collect::<Vec<_>>();

  let output = run_program(&mut numbers.clone(), 1);
  println!("Part 1: {}", output.last().unwrap());

  let output = run_program(&mut numbers.clone(), 5);
  println!("Part 2: {}", output.last().unwrap());
}
