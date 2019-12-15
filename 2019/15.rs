use std::fs::File;
use std::io::prelude::*;
use std::{env, fmt, iter, thread, time};
use std::collections::{HashMap, VecDeque};

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
  Wall,
  Empty,
  Target
}

impl fmt::Display for Tile {
  fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
    write!(f, "{}", match self {
      Tile::Empty => ".",
      Tile::Wall => "#",
      Tile::Target => "F"
    })
  }
}

type Position = (i32, i32);
type TileGrid = HashMap<Position, Tile>;

fn get_input() -> std::io::Result<String> {
  let mut file = File::open("15.txt")?;
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

fn render_screen(tile_grid: &TileGrid, current_position: Position) -> String {
  let mut result = String::new();
  let bounds = tile_grid.keys().cloned()
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
        if current_position == (x, y) {
          result.push('D');
        } else {
          match tile_grid.get(&(x, y)).cloned() {
            Some(tile) => result.push_str(&format!("{}", tile)),
            None => result.push(' ')
          };
        }
      }

      result.push('\n');
    }
  }

  result
}

fn get_known_path(tile_grid: &TileGrid, from: Position, to: Position) -> Option<Vec<Position>> {
  let mut queue = iter::once(from).collect::<VecDeque<_>>();
  let mut previous_map = iter::once((from, None)).collect::<HashMap<_, _>>();

  while let Some(position) = queue.pop_front() {
    if position == to {
      break;
    }

    let neighbors = [(1, 0), (-1, 0), (0, 1), (0, -1)].into_iter()
      .cloned()
      .map(|(dx, dy)| (position.0 + dx, position.1 + dy))
      .filter(|pos| match tile_grid.get(pos) {
        Some(Tile::Wall) | None => false,
        _ => true
      });

    for neighbor in neighbors {
      if previous_map.contains_key(&neighbor) { continue; }

      previous_map.insert(neighbor, Some(position));
      queue.push_back(neighbor);
    }
  }

  let mut result = vec![to];

  while let Some(&Some(previous)) = previous_map.get(result.last().unwrap()) {
    result.push(previous);
  }

  match result.last() {
    Some(&start) if start == from => {
      result.reverse();
      Some(result)
    },
    _ => None
  }
}

fn move_to(
  state: (&mut Vec<i64>, &mut usize, &mut usize),
  tile_grid: &mut TileGrid,
  position: &mut Position,
  target: Position
) -> bool {
  match get_known_path(tile_grid, *position, target) {
    None => false,
    Some(path) => {
      let inputs = path.iter()
        .enumerate()
        .skip(1)
        .map(|(i, &(x, y))| (x - path[i - 1].0, y - path[i - 1].1))
        .map(|diff| match diff {
          (0, -1) => 1,
          (0, 1) => 2,
          (-1, 0) => 3,
          (1, 0) => 4,
          _ => panic!()
        })
        .zip(path.iter().skip(1).cloned());

      for (input, pos) in inputs {
        run_program((state.0, state.1, state.2), Some(input));
        *position = pos;
      }

      true
    }
  }
}

fn discover_target(
  state: (&mut Vec<i64>, &mut usize, &mut usize),
  tile_grid: &mut TileGrid,
  current_position: &mut Position,
  print: bool
) -> Option<Position> {
  let mut target_position = None;
  let mut stack = vec![*current_position];

  while let Some(position) = stack.pop() {
    for &input in &[1, 2, 3, 4] {
      let success = move_to((state.0, state.1, state.2), tile_grid, current_position, position);
      if !success { panic!(); }

      let (x, y) = *current_position;
      let next_position = match input {
        1 => (x, y - 1),
        2 => (x, y + 1),
        3 => (x - 1, y),
        4 => (x + 1, y),
        _ => panic!()
      };

      if tile_grid.contains_key(&next_position) {
        continue;
      }

      let next_tile = match run_program((state.0, state.1, state.2), Some(input)) {
        ProgramResult::Output(0) => Tile::Wall,
        ProgramResult::Output(1) => Tile::Empty,
        ProgramResult::Output(2) => {
          target_position = Some(next_position);
          Tile::Target
        },
        _ => panic!()
      };

      match next_tile {
        Tile::Wall => {
          tile_grid.insert(next_position, Tile::Wall);
        },
        _ => {
          tile_grid.insert(next_position, next_tile);
          *current_position = next_position;
          stack.push(next_position);
        }
      };

      if print {
        println!("{}", render_screen(tile_grid, *current_position));
        thread::sleep(time::Duration::from_millis(50));
      }
    }
  }

  target_position
}

fn get_flood_time(tile_grid: &TileGrid, from: Position) -> Option<u32> {
  let mut queue = iter::once((from, 0)).collect::<VecDeque<_>>();
  let mut time_map = iter::once((from, 0)).collect::<HashMap<_, _>>();

  while let Some((position, time)) = queue.pop_front() {
    let neighbors = [(1, 0), (-1, 0), (0, 1), (0, -1)].into_iter()
      .cloned()
      .map(|(dx, dy)| (position.0 + dx, position.1 + dy))
      .filter(|pos| match tile_grid.get(pos) {
        Some(Tile::Wall) | None => false,
        _ => true
      });

    for neighbor in neighbors {
      if time_map.contains_key(&neighbor) { continue; }

      time_map.insert(neighbor, time + 1);
      queue.push_back((neighbor, time + 1));
    }
  }

  time_map.into_iter()
  .map(|(_, x)| x)
  .max()
}

fn main() {
  let input = get_input().unwrap();
  let program = input.split(',')
    .filter_map(|x| x.trim().parse::<i64>().ok())
    .collect::<Vec<_>>();

  let mut current_position = (0, 0);
  let mut tile_grid = TileGrid::new();
  tile_grid.insert(current_position, Tile::Empty);

  let print = env::args().any(|s| s == "--print");
  let oxygen_system = discover_target(
    (&mut program.clone(), &mut 0, &mut 0),
    &mut tile_grid,
    &mut current_position,
    print
  );

  let path = get_known_path(&tile_grid, (0, 0), oxygen_system.unwrap());

  println!("Part 1: {}", path.unwrap().len() - 1);

  let flood_time = get_flood_time(&tile_grid, oxygen_system.unwrap());

  println!("Part 2: {}", flood_time.unwrap());
}
