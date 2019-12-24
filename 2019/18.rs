use std::fs::File;
use std::io::prelude::*;
use std::collections::HashMap;
use std::hash::Hash;
use std::rc::Rc;
use std::fmt::Debug;

mod graph;
use graph::Graph;

#[derive(Debug, Copy, Clone)]
enum Tile<K> {
  Wall,
  Passage,
  Door(K),
  Key(K)
}

type Position = (usize, usize);
type PositionKeys<K> = (Position, Rc<Vec<K>>);
type PositionsKeys<K> = (Vec<Position>, Rc<Vec<K>>);
type Labyrinth<K> = HashMap<Position, Tile<K>>;

impl<K: Hash + Eq + Clone + Ord> Graph<PositionKeys<K>> for Labyrinth<K> {
  fn get_neighbors(&self, ((x, y), keys): PositionKeys<K>) -> Vec<PositionKeys<K>> {
    [(x - 1, y), (x + 1, y), (x, y - 1), (x, y + 1)].into_iter()
    .cloned()
    .filter(|&(x, y)| x > 0 && y > 0)
    .filter_map(|pos| match self.get(&pos) {
      Some(Tile::Passage) => {
        Some((pos, keys.clone()))
      },
      Some(Tile::Door(x)) if keys.contains(x) => {
        Some((pos, keys.clone()))
      },
      Some(Tile::Key(k)) => {
        Some((pos, {
          let mut keys = (*keys).clone();

          keys.push(k.clone());
          keys.sort();
          keys.dedup();

          Rc::new(keys)
        }))
      },
      _ => None
    })
    .collect()
  }
}

impl<K: Hash + Eq + Clone + Ord> Graph<PositionsKeys<K>> for Labyrinth<K> {
  fn get_neighbors(&self, (positions, keys): PositionsKeys<K>) -> Vec<PositionsKeys<K>> {
    positions.iter().cloned()
    .map(|position| (position, keys.clone()))
    .enumerate()
    .flat_map(|(i, position_keys)| {
      self.get_neighbors(position_keys).into_iter()
      .map(move |x| (i, x))
      .map(|(i, (position, keys))| {
        let mut positions = positions.clone();
        positions[i] = position;

        (positions, keys)
      })
    })
    .collect()
  }
}

fn get_input() -> std::io::Result<String> {
  let mut file = File::open("18.txt")?;
  let mut contents = String::new();
  file.read_to_string(&mut contents)?;

  Ok(contents)
}

fn parse_labyrinth(input: &str) -> (Labyrinth<char>, Option<Position>) {
  let labyrinth = input.lines()
    .enumerate()
    .flat_map(|(y, line)| {
      line.chars()
      .enumerate()
      .map(move |(x, c)| (
        (x, y),
        match c {
          '#' => Tile::Wall,
          '@' => Tile::Passage,
          c if c.is_ascii_uppercase() => Tile::Door(c.to_ascii_lowercase()),
          c if c.is_ascii_lowercase() => Tile::Key(c),
          _ => Tile::Passage
        }
      ))
    })
    .collect::<Labyrinth<_>>();

  let entrance = input.lines()
    .enumerate()
    .find_map(|(y, line)| {
      line.chars()
      .position(|c| c == '@')
      .map(|x| (x, y))
    });

  (labyrinth, entrance)
}

fn get_all_keys<K>(labyrinth: &Labyrinth<K>, position: Position, key_count: usize) -> Option<Vec<Position>>
where K: Hash + Eq + Clone + Ord {
  let mut bfs_iter = labyrinth.bfs((position, Rc::new(vec![])));

  bfs_iter
  .find(|(_, keys)| keys.len() == key_count)
  .and_then(|target| {
    bfs_iter.construct_path(target)
    .map(|path| {
      path.into_iter()
      .map(|(position, _)| position)
      .collect()
    })
  })
}

fn real_get_all_keys<K>(labyrinth: &Labyrinth<K>, positions: Vec<Position>, key_count: usize) -> Option<Vec<Vec<Position>>>
where K: Hash + Eq + Clone + Ord + Debug {
  let mut bfs_iter = labyrinth.bfs((positions, Rc::new(vec![])));

  bfs_iter
  .find(|(_, keys)| {
    if keys.len() > 5 {
      println!("{:?}", keys);
    }
    keys.len() == key_count
  })
  .and_then(|target| {
    bfs_iter.construct_path(target)
    .map(|path| {
      path.into_iter()
      .map(|(positions, _)| positions)
      .collect()
    })
  })
}

fn main() {
  let input = get_input().unwrap();
  let (mut labyrinth, position) = parse_labyrinth(&input);

  let path = get_all_keys(&labyrinth, position.unwrap(), 26);
  println!("Part 1: {}", path.unwrap().len() - 1);

  let (x, y) = position.unwrap();
  labyrinth.insert((x, y), Tile::Wall);
  labyrinth.insert((x - 1, y), Tile::Wall);
  labyrinth.insert((x + 1, y), Tile::Wall);
  labyrinth.insert((x, y - 1), Tile::Wall);
  labyrinth.insert((x, y + 1), Tile::Wall);

  let positions = vec![(x - 1, y - 1), (x + 1, y - 1), (x - 1, y + 1), (x + 1, y + 1)];
  let path = real_get_all_keys(&labyrinth, positions, 26);

  println!("Part 2: {}", path.unwrap().len() - 1);
}
