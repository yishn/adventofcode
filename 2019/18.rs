use std::fs::File;
use std::io::prelude::*;
use std::collections::{HashMap, VecDeque};
use std::hash::Hash;

#[derive(Debug, Copy, Clone)]
enum Tile<T> {
  Wall,
  Passage,
  Door(T),
  Key(T)
}

type Position = (usize, usize);
type Labyrinth<T> = HashMap<Position, Tile<T>>;

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

fn get_neighbors<T: Hash + Eq>(labyrinth: &Labyrinth<T>, (x, y): Position, keys: &Vec<T>) -> Vec<Position> {
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

fn get_all_keys(labyrinth: &Labyrinth<char>, position: Position, key_count: usize) -> Vec<Position> {
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

    for neighbor in get_neighbors(labyrinth, position, &keys) {
      let mut new_keys = keys.clone();

      if let Some(&Tile::Key(c)) = labyrinth.get(&neighbor) {
        new_keys.push(c);
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

  let mut result = vec![target.unwrap()];

  while let Some(Some(prev)) = previous_map.get(&result.last().unwrap()) {
    result.push(prev.clone());
  }

  result.into_iter().rev().map(|(position, _)| position).collect()
}

fn main() {
  let input = get_input().unwrap();
  let (labyrinth, position) = parse_labyrinth(&input);

  let path = get_all_keys(&labyrinth, position.unwrap(), 26);

  println!("Part 1: {}", path.len() - 1);
}
