use std::fs::File;
use std::io::prelude::*;
use std::collections::HashMap;

#[derive(Debug, Clone, Copy)]
enum Direction {
  Left,
  Right,
  Up,
  Down
}

type Wire = Vec<(Direction, usize)>;
type Grid = HashMap<(i32, i32), HashMap<usize, usize>>;

fn get_input() -> std::io::Result<String> {
  let mut file = File::open("03.txt")?;
  let mut contents = String::new();
  file.read_to_string(&mut contents)?;

  Ok(contents)
}

fn norm((x, y): (i32, i32)) -> i32 {
  x.abs() + y.abs()
}

fn parse_wires(input: &str) -> Vec<Wire> {
  input.lines()
  .map(|line| {
    line.split(',')
    .filter_map(|token| {
      if token.len() < 2 {
        return None;
      }

      let steps = token.get(1..).and_then(|steps| steps.parse::<usize>().ok());
      let direction = match token.chars().nth(0) {
        Some('L') => Some(Direction::Left),
        Some('R') => Some(Direction::Right),
        Some('U') => Some(Direction::Up),
        Some('D') => Some(Direction::Down),
        _ => None
      };

      match (direction, steps) {
        (Some(direction), Some(steps)) => Some((direction, steps)),
        _ => None
      }
    })
    .collect::<Vec<_>>()
  })
  .collect::<Vec<_>>()
}

fn trace_wire(grid: &mut Grid, id: usize, wire: &Wire) {
  let mut position = (0, 0);
  let mut timestamp = 0;

  for &(direction, steps) in wire.iter() {
    let move_vector = match direction {
      Direction::Left => (-1, 0),
      Direction::Right => (1, 0),
      Direction::Up => (0, 1),
      Direction::Down => (0, -1)
    };

    for _ in 0..steps {
      position = (position.0 + move_vector.0, position.1 + move_vector.1);
      timestamp += 1;

      if !grid.contains_key(&position) {
        grid.insert(position, HashMap::new());
      }

      let ids = grid.get_mut(&position).unwrap();

      if !ids.contains_key(&id) {
        ids.insert(id, timestamp);
      }
    }
  }
}

fn iter_collisions<'a>(grid: &'a Grid) -> impl Iterator<Item = (i32, i32)> + 'a {
  grid.iter()
  .filter(|(_, ids)| ids.len() > 1)
  .map(|(&pos, _)| pos)
}

fn iter_collision_times<'a>(grid: &'a Grid) -> impl Iterator<Item = usize> + 'a {
  iter_collisions(grid)
  .map(move |pos| {
    grid.get(&pos)
    .unwrap()
    .values()
    .sum()
  })
}

fn main() {
  let input = get_input().unwrap();
  let wires = parse_wires(&input);
  let mut grid = Grid::new();

  for (i, wire) in wires.iter().enumerate() {
    trace_wire(&mut grid, i, wire);
  }

  iter_collisions(&grid)
  .map(|pos| norm(pos))
  .min()
  .map(|min_collision_norm| {
    println!("Part 1: {}", min_collision_norm);
  });

  iter_collision_times(&grid)
  .min()
  .map(|min_collision_time| {
    println!("Part 2: {}", min_collision_time);
  });
}
