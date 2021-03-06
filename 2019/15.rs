use std::fs::File;
use std::io::prelude::*;
use std::{env, fmt, iter, thread, time};
use std::collections::{HashMap, VecDeque};

mod intcode;
use intcode::{ProgramState, ProgramResult, run_program};

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
  state: &mut ProgramState,
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
        run_program(state, Some(input));
        *position = pos;
      }

      true
    }
  }
}

fn discover_target(
  state: &mut ProgramState,
  tile_grid: &mut TileGrid,
  current_position: &mut Position,
  print: bool
) -> Option<Position> {
  let mut target_position = None;
  let mut stack = vec![*current_position];

  while let Some(position) = stack.pop() {
    for &input in &[1, 2, 3, 4] {
      let success = move_to(state, tile_grid, current_position, position);
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

      let next_tile = match run_program(state, Some(input)) {
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
    &mut ProgramState::new(program.clone()),
    &mut tile_grid,
    &mut current_position,
    print
  );

  let path = get_known_path(&tile_grid, (0, 0), oxygen_system.unwrap());

  println!("Part 1: {}", path.unwrap().len() - 1);

  let flood_time = get_flood_time(&tile_grid, oxygen_system.unwrap());

  println!("Part 2: {}", flood_time.unwrap());
}
