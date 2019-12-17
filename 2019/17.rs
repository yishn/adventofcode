use std::fs::File;
use std::io::prelude::*;
use std::collections::HashMap;
use std::{env, iter};

mod intcode;
use intcode::{ProgramResult, ProgramState, run_program, run_program_with_inputs};

#[derive(Debug, Copy, Clone)]
enum Tile {
  Scaffold,
  Space
}

#[derive(Debug, Copy, Clone, PartialEq)]
enum Movement {
  TurnLeft,
  TurnRight,
  Forward(isize)
}

type Position = (isize, isize);
type Direction = (isize, isize);
type ScaffoldsMap = HashMap<Position, Tile>;
type WorldState = (ScaffoldsMap, Position, Direction);

fn get_input() -> std::io::Result<String> {
  let mut file = File::open("17.txt")?;
  let mut contents = String::new();
  file.read_to_string(&mut contents)?;

  Ok(contents)
}

fn get_world_state(state: &mut ProgramState) -> WorldState {
  let mut map = ScaffoldsMap::new();
  let (mut position, mut direction) = (None, None);
  let (mut x, mut y) = (0, 0);

  while let ProgramResult::Output(c) = run_program(state, None) {
    let tile = match c as u8 as char {
      '#' | '<' | '>' | 'v' | '^' => Some(Tile::Scaffold),
      '.' => Some(Tile::Space),
      '\n' => None,
      _ => panic!()
    };

    if let None = direction {
      direction = match c as u8 as char {
        '<' => Some((-1, 0)),
        '>' => Some((1, 0)),
        '^' => Some((0, -1)),
        'v' => Some((0, 1)),
        _ => None
      };

      if let Some(_) = direction {
        position = Some((x, y));
      }
    }

    if let Some(tile) = tile {
      map.insert((x, y), tile);
      x += 1;
    } else {
      x = 0;
      y += 1;
    }
  }

  (map, position.unwrap(), direction.unwrap())
}

fn render_world((map, position, direction): &WorldState) -> String {
  let mut result = String::new();
  let bounds = map.keys().cloned()
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
        result.push(
          if *position == (x, y) {
            match direction {
              (-1, 0) => '<',
              (1, 0) => '>',
              (0, -1) => '^',
              (0, 1) => 'v',
              _ => 'X'
            }
          } else {
            match map.get(&(x, y)) {
              Some(Tile::Scaffold) => '#',
              Some(Tile::Space) => '.',
              None => ' ',
            }
          }
        );
      }

      result.push('\n');
    }
  }

  result
}

fn get_intersections(map: &ScaffoldsMap) -> Vec<Position> {
  map.keys()
  .cloned()
  .filter(|&(x, y)| x > 0 && y > 0)
  .filter(|&(x, y)| {
    [(x, y), (x - 1, y), (x + 1, y), (x, y - 1), (x, y + 1)]
    .into_iter()
    .all(|neighbor| match map.get(neighbor) {
      Some(Tile::Scaffold) => true,
      _ => false
    })
  })
  .collect()
}

fn get_full_movement_instruction((map, position, direction): &WorldState) -> Vec<Movement> {
  let mut result = vec![];
  let (mut position, mut direction) = (*position, *direction);

  loop {
    let next_position = (position.0 + direction.0, position.1 + direction.1);

    if let Some(&Tile::Scaffold) = map.get(&next_position) {
      // Move forward

      if let Some(Movement::Forward(x)) = result.last_mut() {
        *x += 1;
      } else {
        result.push(Movement::Forward(1));
      }

      position = next_position;
    } else {
      // Turn

      let turn_info = [((0, 1), Movement::TurnRight), ((0, -1), Movement::TurnLeft)]
        .into_iter()
        .cloned()
        .map(|((x, y), m)| {
          ((x * direction.0 - y * direction.1, x * direction.1 + y * direction.0), m)
        })
        .find(|((dx, dy), _)| match map.get(&(position.0 + dx, position.1 + dy)) {
          Some(&Tile::Scaffold) => true,
          _ => false
        });

      direction = match turn_info {
        Some((d, movement)) => {
          result.push(movement);
          d
        },
        _ => break
      };
    }
  }

  result
}

fn compress_movement_instruction(
  instruction: &[Movement],
  chunk_count: usize,
  chunk_size: usize
) -> Option<(Vec<&[Movement]>, Vec<usize>)> {
  fn compress_movement_instruction_inner<'a>(
    instruction: &'a [Movement],
    chunk_count: usize,
    chunk_size: usize,
    chunks: Vec<&'a [Movement]>,
    chunk_instruction: Vec<usize>
  ) -> Option<(Vec<&'a [Movement]>, Vec<usize>)> {
    if chunks.len() > chunk_count || chunk_instruction.len() > chunk_size {
      return None;
    }

    if instruction.len() == 0 {
      return Some((chunks, chunk_instruction));
    }

    (1..=chunk_size).filter_map(|size| {
      let new_chunk = &instruction[0..size];
      let existing_chunk = chunks.iter()
        .position(|&chunk| chunk == new_chunk);

      let mut new_chunk_instruction = chunk_instruction.clone();
      let mut new_chunks = chunks.clone();

      if let Some(index) = existing_chunk {
        new_chunk_instruction.push(index);
      } else {
        new_chunks.push(new_chunk);
        new_chunk_instruction.push(chunks.len());
      }

      compress_movement_instruction_inner(
        &instruction[size..],
        chunk_count,
        chunk_size,
        new_chunks,
        new_chunk_instruction
      )
    })
    .next()
  }

  let chunks = vec![];
  let chunk_instruction = vec![];

  compress_movement_instruction_inner(instruction, chunk_count, chunk_size, chunks, chunk_instruction)
}

fn main() {
  let input = get_input().unwrap();
  let program = input.split(',')
    .filter_map(|x| x.trim().parse::<i64>().ok())
    .collect::<Vec<_>>();

  let world = get_world_state(&mut ProgramState::new(program.clone()));
  let intersections = get_intersections(&world.0);
  let total_alignment_parameter = intersections.into_iter()
    .map(|(x, y)| x * y)
    .sum::<isize>();

  println!("Part 1: {}", total_alignment_parameter);

  if env::args().any(|s| s == "--print") {
    println!("{}", render_world(&world));
  }

  let mut program = program;
  program[0] = 2;

  let instructions = get_full_movement_instruction(&world);
  let (chunks, chunk_instruction) = compress_movement_instruction(&instructions, 3, 10).unwrap();

  let inputs = chunk_instruction.into_iter()
    .flat_map(|index| vec![
      ',',
      match index {
        0 => 'A',
        1 => 'B',
        2 => 'C',
        _ => panic!()
      }
    ])
    .skip(1)
    .chain(iter::once('\n'))
    .chain(chunks.into_iter().flat_map(|chunk| {
      chunk.iter()
      .cloned()
      .flat_map(|movement| {
        iter::once(',')
        .chain(match movement {
          Movement::Forward(x) => x.to_string().chars().collect(),
          Movement::TurnLeft => vec!['L'],
          Movement::TurnRight => vec!['R']
        })
      })
      .skip(1)
      .chain(iter::once('\n'))
    }))
    .chain(vec!['n', '\n'])
    .map(|x| x as u8 as i64);

  let outputs = run_program_with_inputs(&mut ProgramState::new(program), inputs);

  println!("Part 2: {}", outputs.last().unwrap());
}
