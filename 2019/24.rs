use std::fs::File;
use std::io::prelude::*;
use std::collections::{HashMap, HashSet};
use std::hash::Hash;

mod graph;
use graph::Graph;

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
enum Tile {
  Bug,
  Empty
}

type Position = (usize, usize);
type RecursivePosition = (i64, Position);

struct TileMap<P>(HashMap<P, Tile>);

impl<P: Hash + Eq + Copy> TileMap<P> {
  fn new() -> TileMap<P> {
    TileMap(HashMap::new())
  }

  fn tick(&self) -> TileMap<P> where Self: Graph<P> {
    let mut result = TileMap::new();

    let considered_positions = self.0.iter()
      .flat_map(|(&pos, _)| self.get_neighbors(pos))
      .filter(|pos| !self.0.contains_key(&pos))
      .chain(self.0.keys().cloned());

    for position in considered_positions {
      let neighbors = self.get_neighbors(position);
      let bug_neighbor_count = neighbors.iter()
        .filter(|&pos| self.0.get(pos) == Some(&Tile::Bug))
        .count();

      let tile = match self.0.get(&position) {
        Some(&x) => x,
        _ => Tile::Empty
      };

      let new_tile = match (tile, bug_neighbor_count) {
        (Tile::Bug, 1) | (Tile::Empty, 1) | (Tile::Empty, 2) => Tile::Bug,
        _ => Tile::Empty
      };

      result.0.insert(position, new_tile);
    }

    result
  }

  fn count_bugs(&self) -> usize {
    self.0.values()
    .filter(|&tile| tile == &Tile::Bug)
    .count()
  }
}

impl TileMap<Position> {
  fn get_size(&self) -> (usize, usize) {
    let size = self.0.keys().cloned()
      .fold((None, None), |(max_x, max_y), (x, y)| {
        (
          [max_x, Some(x)].into_iter().filter_map(|&x| x).max(),
          [max_y, Some(y)].into_iter().filter_map(|&y| y).max()
        )
      });

    match size {
      (Some(x), Some(y)) => (x + 1, y + 1),
      _ => (0, 0)
    }
  }

  fn render(&self) -> String {
    let mut result = String::new();
    let (width, height) = self.get_size();

    for y in 0..height {
      for x in 0..width {
        result.push(match self.0.get(&(x, y)) {
          Some(&Tile::Bug) => '#',
          _ => '.'
        });
      }

      result.push('\n');
    }

    result
  }

  fn get_biodiversity_rating(&self) -> u64 {
    let (width, height) = self.get_size();

    (0..height)
    .flat_map(|y| (0..width).map(move |x| (x, y)))
    .enumerate()
    .filter(|&(_, pos)| match self.0.get(&pos) {
      Some(&Tile::Bug) => true,
      _ => false
    })
    .map(|(i, _)| 2u64.pow(i as u32))
    .sum::<u64>()
  }
}

impl Graph<Position> for TileMap<Position> {
  fn get_neighbors(&self, (x, y): Position) -> Vec<Position> {
    if !self.0.contains_key(&(x, y)) {
      return vec![];
    }

    let (x, y) = (x as isize, y as isize);

    [(x - 1, y), (x + 1, y), (x, y - 1), (x, y + 1)].into_iter().cloned()
    .filter(|&(x, y)| x >= 0 && y >= 0)
    .map(|(x, y)| (x as usize, y as usize))
    .filter(|pos| self.0.contains_key(pos))
    .collect()
  }
}

impl TileMap<RecursivePosition> {
  fn from_initial_level(initial_level: &TileMap<Position>) -> TileMap<RecursivePosition> {
    TileMap(
      initial_level.0.iter()
      .filter(|&(&pos, _)| pos != (2, 2))
      .map(|(&pos, &tile)| ((0, pos), tile))
      .collect()
    )
  }
}

impl Graph<RecursivePosition> for TileMap<RecursivePosition> {
  fn get_neighbors(&self, position: RecursivePosition) -> Vec<RecursivePosition> {
    let (level, (x, y)) = position;
    let (x, y) = (x as isize, y as isize);

    [(x - 1, y), (x + 1, y), (x, y - 1), (x, y + 1)].into_iter().cloned()
    .map(|pos| (level, pos))
    .flat_map(|(level, (nx, ny))| {
      if (nx, ny) == (2, 2) {
        (0..5).map(|i| {
          (
            level + 1,
            if x < 2 { (0, i) } else if x > 2 { (4, i) }
            else if y < 2 { (i, 0) } else if y > 2 { (i, 4) }
            else { panic!() }
          )
        })
        .collect()
      } else if nx >= 0 && ny >= 0 && nx < 5 && ny < 5 {
        vec![(level, (nx as usize, ny as usize))]
      } else if nx < 0 {
        vec![(level - 1, (1, 2))]
      } else if ny < 0 {
        vec![(level - 1, (2, 1))]
      } else if nx >= 5 {
        vec![(level - 1, (3, 2))]
      } else if ny >= 5 {
        vec![(level - 1, (2, 3))]
      } else {
        panic!()
      }
    })
    .collect()
  }
}

fn get_input() -> std::io::Result<String> {
  let mut file = File::open("24.txt")?;
  let mut contents = String::new();
  file.read_to_string(&mut contents)?;

  Ok(contents)
}

fn parse_input(input: &str) -> TileMap<Position> {
  TileMap(
    input.lines().enumerate()
    .flat_map(|(y, line)| {
      line.chars().enumerate()
      .filter_map(move |(x, c)| {
        match c {
          '#' => Some(Tile::Bug),
          '.' => Some(Tile::Empty),
          _ => None
        }.map(|tile| {
          ((x, y), tile)
        })
      })
    })
    .collect()
  )
}

fn main() {
  let input = get_input().unwrap();
  let mut map = parse_input(&input);
  let mut history = std::iter::once(map.render()).collect::<HashSet<_>>();

  loop {
    map = map.tick();
    let render = map.render();

    if history.contains(&render) {
      println!("Part 1: {}", map.get_biodiversity_rating());
      break;
    }

    history.insert(render);
  }

  let map = parse_input(&input);
  let mut recursive_map = TileMap::from_initial_level(&map);

  for _ in 0..200 {
    recursive_map = recursive_map.tick();
  }

  println!("Part 2: {}", recursive_map.count_bugs());
}
