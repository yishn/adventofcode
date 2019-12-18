use std::fs::File;
use std::io::prelude::*;
use std::collections::{HashMap, VecDeque};
use std::hash::Hash;

#[derive(Debug, Copy, Clone)]
enum Tile<K> {
  Wall,
  Passage,
  Door(K),
  Key(K)
}

type Position = (usize, usize);
type Labyrinth<K> = HashMap<Position, Tile<K>>;

trait HasNeighbors<K> {
  fn neighbors(&self, labyrinth: &Labyrinth<K>, keys: &[K]) -> Vec<Self> where Self: Sized;
}

impl<K> HasNeighbors<K> for Position
where K: Hash + Eq {
  fn neighbors(&self, labyrinth: &Labyrinth<K>, keys: &[K]) -> Vec<Position> {
    let &(x, y) = self;

    [(x - 1, y), (x + 1, y), (x, y - 1), (x, y + 1)].into_iter()
    .cloned()
    .filter(|&(x, y)| x > 0 && y > 0)
    .filter(|pos| match labyrinth.get(pos) {
      Some(Tile::Passage) | Some(Tile::Key(_)) => true,
      Some(Tile::Door(x)) => keys.contains(x),
      _ => false
    })
    .collect()
  }
}

impl<K> HasNeighbors<K> for Vec<Position>
where K: Hash + Eq + Clone {
  fn neighbors(&self, labyrinth: &Labyrinth<K>, keys: &[K]) -> Vec<Vec<Position>> {
    self.iter()
    .cloned()
    .enumerate()
    .flat_map(|(i, position)| {
      position.neighbors(labyrinth, keys).into_iter()
      .map(move |neighbor| {
        let mut positions = self.iter().cloned().collect::<Vec<_>>();
        positions[i] = neighbor;
        positions
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
  let mut queue = VecDeque::new();
  let mut previous_map = HashMap::new();
  let mut target = None;

  queue.push_back((position, vec![]));
  previous_map.insert((position, vec![]), None);

  while let Some((position, keys)) = queue.pop_front() {
    if keys.len() >= key_count {
      target = Some((position, keys));
      break;
    }

    for neighbor in position.neighbors(labyrinth, &keys) {
      let mut new_keys = keys.clone();

      if let Some(Tile::Key(c)) = labyrinth.get(&neighbor) {
        new_keys.push(c.clone());
        new_keys.sort();
        new_keys.dedup();
      }

      let map_key = (neighbor, new_keys);

      if !previous_map.contains_key(&map_key) {
        queue.push_back((neighbor, map_key.1.clone()));
        previous_map.insert(map_key, Some((position, keys.clone())));
      }
    }
  }

  target.map(|target| {
    let mut result = vec![target];

    while let Some(Some(prev)) = previous_map.get(&result.last().unwrap()) {
      result.push(prev.clone());
    }

    result.into_iter().rev().map(|(position, _)| position).collect()
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
}
