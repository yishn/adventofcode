use std::fs::File;
use std::io::prelude::*;
use std::collections::HashMap;
use std::{fmt, env, thread, time};

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

#[derive(Debug, Copy, Clone)]
enum ProgramResult {
  Output(i64),
  WaitForInput,
  Halt,
}

#[derive(Debug, PartialEq, Copy, Clone)]
enum Tile {
  Empty,
  Wall,
  Block,
  HorizontalPaddle,
  Ball
}

impl fmt::Display for Tile {
  fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
    write!(f, "{}", match self {
      Tile::Empty => "  ",
      Tile::Wall => "##",
      Tile::Block => "[]",
      Tile::HorizontalPaddle => "==",
      Tile::Ball => "()",
    })
  }
}

type TileGrid = HashMap<(i64, i64), Tile>;

fn get_input() -> std::io::Result<String> {
  let mut file = File::open("13.txt")?;
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

fn output_screen(state: (&mut Vec<i64>, &mut usize, &mut usize), input: Option<i64>) -> (TileGrid, Option<i64>, bool) {
  let mut result = TileGrid::new();
  let mut score = None;
  let mut input_iter = input.into_iter();
  let mut halted = false;

  loop {
    let mut get = |input| run_program((state.0, state.1, state.2), input);

    match (get(input_iter.next()), get(None), get(None)) {
      (ProgramResult::Output(x), ProgramResult::Output(y), ProgramResult::Output(value)) => {
        if (x, y) == (-1, 0) {
          score = Some(value);
        } else {
          result.insert((x, y), match value {
            0 => Tile::Empty,
            1 => Tile::Wall,
            2 => Tile::Block,
            3 => Tile::HorizontalPaddle,
            4 => Tile::Ball,
            _ => panic!()
          });
        }
      },
      (ProgramResult::WaitForInput, _, _) => break,
      (ProgramResult::Halt, _, _) => {
        halted = true;
        break;
      },
      _ => panic!()
    };
  }

  (result, score, halted)
}

fn render_screen(screen: &TileGrid) -> String {
  let mut result = String::new();
  let bounds = screen.keys().cloned()
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
      for x in min_x..=max_x {
        let tile = screen.get(&(x, y)).cloned().unwrap_or(Tile::Empty);

        result.push_str(&format!("{}", tile));
      }

      result.push('\n');
    }
  }

  result
}

fn play_game(state: (&mut Vec<i64>, &mut usize, &mut usize), print: bool) -> i64 {
  fn get_tile_position(screen: &TileGrid, tile: Tile) -> (i64, i64) {
    screen.iter()
    .find(|&(_, &t)| t == tile)
    .map(|(&position, _)| position)
    .unwrap()
  }

  let (mut screen, mut score, _) = output_screen((state.0, state.1, state.2), None);
  let mut joystick = 0;

  loop {
    let (screen_update, score_update, halted) = output_screen((state.0, state.1, state.2), Some(joystick));

    for (&(x, y), &tile) in screen_update.iter() {
      screen.insert((x, y), tile);
    }

    if let Some(_) = score_update {
      score = score_update;
    }

    let paddle_position = get_tile_position(&screen, Tile::HorizontalPaddle);
    let ball_position = get_tile_position(&screen, Tile::Ball);

    if ball_position.0 == paddle_position.0 {
      joystick = 0;
    } else if ball_position.0 < paddle_position.0 {
      joystick = -1;
    } else {
      joystick = 1;
    }

    if print {
      println!("// Score: {}\n{}\n", score.unwrap_or(0), render_screen(&screen));
      thread::sleep(time::Duration::from_millis(20));
    }

    if halted {
      break;
    }
  }

  score.unwrap_or(0)
}

fn main() {
  let input = get_input().unwrap();
  let numbers = input.split(',')
    .filter_map(|x| x.trim().parse::<i64>().ok())
    .collect::<Vec<_>>();

  let (screen, _, _) = output_screen((&mut numbers.clone(), &mut 0, &mut 0), None);
  let block_tile_count = screen.iter()
    .filter(|&(_, &tile)| match tile {
      Tile::Block => true,
      _ => false
    })
    .count();

  println!("Part 1: {}", block_tile_count);

  let mut program = numbers.clone();
  program[0] = 2;

  let print_game = env::args().any(|s| s == "--print");
  let score = play_game((&mut program, &mut 0, &mut 0), print_game);

  println!("Part 2: {}", score);
}
