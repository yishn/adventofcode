use std::fs::File;
use std::io::prelude::*;
use std::collections::HashMap;

#[derive(Debug)]
enum ParameterMode {
  Position,
  Immediate,
  Relative
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
  SetRelativeBase,
  Halt
}

#[derive(Debug)]
struct Instruction {
  operation: OperationType,
  inputs: Vec<(ParameterMode, i64)>
}

#[derive(Debug, PartialEq, Copy, Clone)]
enum ProgramResult {
  Output(i64),
  WaitForInput,
  Halt,
}

#[derive(Debug, Copy, Clone)]
enum Color {
  Black,
  White
}

#[derive(Debug)]
enum BotInstruction {
  TurnLeft,
  TurnRight
}

type Hull = HashMap<(i32, i32), Color>;

fn get_input() -> std::io::Result<String> {
  let mut file = File::open("11.txt")?;
  let mut contents = String::new();
  file.read_to_string(&mut contents)?;

  Ok(contents)
}

fn parse_instruction(numbers: &[i64]) -> Instruction {
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
    9 => (OperationType::SetRelativeBase, 1),
    99 => (OperationType::Halt, 0),
    _ => panic!()
  };

  let get_parameter_mode = |i| {
    match (instruction_code - op_code) / 10i64.pow(2 + i as u32) % 10 {
      0 => ParameterMode::Position,
      1 => ParameterMode::Immediate,
      2 => ParameterMode::Relative,
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

fn run_program(state: (&mut Vec<i64>, &mut usize, &mut usize), input: Option<i64>) -> ProgramResult {
  let (program, pointer, relative_base) = state;
  let mut input = input;

  fn extend_memory(program: &mut Vec<i64>, index: usize) {
    while index >= program.len() {
      program.push(0);
    }
  }

  fn get_instruction_input_index(
    program: &mut Vec<i64>,
    instruction: &Instruction,
    relative_base: usize,
    index: usize
  ) -> usize {
    match instruction.inputs[index] {
      (ParameterMode::Relative, d) => {
        let j = (relative_base as i64 + d) as usize;
        extend_memory(program, j);
        j
      },
      (_, j) => {
        extend_memory(program, j as usize);
        j as usize
      }
    }
  }

  fn get_instruction_input(
    program: &mut Vec<i64>,
    instruction: &Instruction,
    relative_base: usize,
    index: usize
  ) -> i64 {
    match instruction.inputs[index] {
      (ParameterMode::Immediate, value) => value,
      _ => {
        let j = get_instruction_input_index(program, instruction, relative_base, index);
        program[j]
      }
    }
  }

  while *pointer < program.len() {
    let init_pointer = *pointer;
    let instruction = parse_instruction(&program[*pointer..]);

    let (target_value, target_index) = {
      let mut get_input = |i| {
        get_instruction_input(program, &instruction, *relative_base, i)
      };

      let (target_value, output_index) = match instruction.operation {
        OperationType::Add => (Some(get_input(0) + get_input(1)), Some(2)),
        OperationType::Multiply => (Some(get_input(0) * get_input(1)), Some(2)),
        OperationType::Input => match input {
          Some(x) => {
            input = None;
            (Some(x), Some(0))
          },
          _ => return ProgramResult::WaitForInput
        },
        OperationType::Output => (Some(get_input(0)), None),
        OperationType::LessThan => (Some((get_input(0) < get_input(1)) as i64), Some(2)),
        OperationType::Equals => (Some((get_input(0) == get_input(1)) as i64), Some(2)),
        OperationType::JumpIfTrue => {
          if get_input(0) != 0 {
            *pointer = get_input(1) as usize;
          }

          (None, None)
        },
        OperationType::JumpIfFalse => {
          if get_input(0) == 0 {
            *pointer = get_input(1) as usize;
          }

          (None, None)
        },
        OperationType::SetRelativeBase => {
          *relative_base = (*relative_base as i64 + get_input(0)) as usize;
          (None, None)
        }
        OperationType::Halt => break
      };

      (
        target_value,
        output_index.map(|i| {
          get_instruction_input_index(program, &instruction, *relative_base, i)
        })
      )
    };

    if *pointer == init_pointer {
      *pointer += instruction.inputs.len() + 1;
    }

    if let Some(target_value) = target_value {
      if let Some(target_index) = target_index {
        extend_memory(program, target_index);
        program[target_index] = target_value;
      } else {
        return ProgramResult::Output(target_value);
      }
    }
  }

  ProgramResult::Halt
}

fn paint_emergency_hull(hull: &mut Hull, program: &mut Vec<i64>) {
  let mut position = (0, 0);
  let mut direction = (0, -1);
  let mut state = (0, 0);

  fn gauss_mul((a, b): (i32, i32), (c, d): (i32, i32)) -> (i32, i32) {
    (a * c - b * d, a * d + b * c)
  }

  loop {
    let input = match hull.get(&position) {
      Some(&Color::White) => 1,
      _ => 0
    };

    let color = match run_program((program, &mut state.0, &mut state.1), Some(input)) {
      ProgramResult::Halt => break,
      ProgramResult::WaitForInput => panic!(),
      ProgramResult::Output(x) => match x {
        0 => Color::Black,
        1 => Color::White,
        _ => panic!()
      }
    };

    let instruction = match run_program((program, &mut state.0, &mut state.1), None) {
      ProgramResult::Halt => break,
      ProgramResult::WaitForInput => panic!(),
      ProgramResult::Output(x) => match x {
        0 => BotInstruction::TurnLeft,
        1 => BotInstruction::TurnRight,
        _ => panic!()
      }
    };

    hull.insert(position, color);
    direction = match instruction {
      BotInstruction::TurnLeft => gauss_mul(direction, (0, 1)),
      BotInstruction::TurnRight => gauss_mul(direction, (0, -1))
    };

    position = (position.0 + direction.0, position.1 + direction.1);
  }
}

fn render_hull(hull: &Hull) -> String {
  let mut result = String::new();
  let bounds = hull.keys().cloned()
    .fold((None, None, None, None), |(min_x, max_x, min_y, max_y), (x, y)| {
      (
        [min_x, Some(x)].into_iter().filter_map(|&x| x).min(),
        [max_x, Some(x)].into_iter().filter_map(|&x| x).max(),
        [min_y, Some(y)].into_iter().filter_map(|&y| y).min(),
        [max_y, Some(y)].into_iter().filter_map(|&y| y).max()
      )
    });

  if let (Some(min_x), Some(max_x), Some(min_y), Some(max_y)) = bounds {
    for y in min_y..=max_y {
      for x in (min_x..=max_x).rev() {
        let color = hull.get(&(x, y)).cloned().unwrap_or(Color::Black);

        result.push(match color {
          Color::Black => ' ',
          Color::White => '#'
        });
      }

      result.push('\n');
    }
  }

  result
}

fn main() {
  let input = get_input().unwrap();
  let numbers = input.split(',')
    .filter_map(|x| x.trim().parse::<i64>().ok())
    .collect::<Vec<_>>();

  let mut hull = Hull::new();
  paint_emergency_hull(&mut hull, &mut numbers.clone());

  println!("Part 1: {}", hull.len());

  let mut hull = Hull::new();
  hull.insert((0, 0), Color::White);
  paint_emergency_hull(&mut hull, &mut numbers.clone());

  println!("Part 2:\n{}", render_hull(&hull));
}
