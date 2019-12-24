use std::fs::File;
use std::io::prelude::*;
use std::collections::HashMap;
use std::hash::Hash;
use std::rc::Rc;

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

fn get_all_keys<K>(labyrinth: &Labyrinth<K>, position_key: PositionKeys<K>, key_count: usize) -> Option<Vec<Position>>
where K: Hash + Eq + Clone + Ord {
  let mut bfs_iter = labyrinth.bfs(position_key);

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

fn get_size<K>(labyrinth: &Labyrinth<K>) -> (usize, usize) {
  let size = labyrinth.keys().cloned()
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

fn real_get_all_keys<K>(
  labyrinth: &Labyrinth<K>,
  (x, y): Position,
  key_count: usize
) -> (Option<Vec<Position>>, Option<Vec<Position>>, Option<Vec<Position>>, Option<Vec<Position>>)
where K: Hash + Eq + Clone + Ord {
  let (width, height) = get_size(labyrinth);
  let positions = vec![(x - 1, y - 1), (x + 1, y - 1), (x - 1, y + 1), (x + 1, y + 1)];

  let keys = labyrinth.iter()
    .filter_map(|(&position, tile)| match tile {
      Tile::Key(k) => Some((position, k.clone())),
      _ => None
    })
    .collect::<Vec<_>>();

  let position_keys = positions.into_iter()
    .enumerate()
    .map(|(i, position)| {
      let ignore_keys = keys.iter().cloned()
        .filter(|&((x, y), _)| {
          if i == 0 {
            x > width / 2 || y > height / 2
          } else if i == 1 {
            x <= width / 2 || y > height / 2
          } else if i == 2 {
            x > width / 2 || y <= height / 2
          } else if i == 3 {
            x <= width / 2 || y <= height / 2
          } else {
            panic!()
          }
        })
        .map(|(_, k)| k.clone())
        .collect::<Vec<_>>();

      (position, Rc::new(ignore_keys))
    });

  let mut results = position_keys
    .filter_map(|position_key| get_all_keys(labyrinth, position_key, key_count));

  (results.next(), results.next(), results.next(), results.next())
}

fn main() {
  let input = get_input().unwrap();
  let (mut labyrinth, position) = parse_labyrinth(&input);

  let path = get_all_keys(&labyrinth, (position.unwrap(), Rc::new(vec![])), 26);
  println!("Part 1: {}", path.unwrap().len() - 1);

  let (x, y) = position.unwrap();
  labyrinth.insert((x, y), Tile::Wall);
  labyrinth.insert((x - 1, y), Tile::Wall);
  labyrinth.insert((x + 1, y), Tile::Wall);
  labyrinth.insert((x, y - 1), Tile::Wall);
  labyrinth.insert((x, y + 1), Tile::Wall);

  let (p1, p2, p3, p4) = real_get_all_keys(&labyrinth, (x, y), 26);
  println!("Part 2: {}", p1.unwrap().len() + p2.unwrap().len() + p3.unwrap().len() + p4.unwrap().len() - 4);
}
