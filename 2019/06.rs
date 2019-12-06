use std::fs::File;
use std::io::prelude::*;
use std::collections::{HashMap, HashSet, VecDeque};
use std::hash::Hash;

#[derive(Debug)]
struct OrbitMap<T: Hash + Eq> {
  objects: HashSet<T>,
  parents: HashMap<T, T>,
  children: HashMap<T, Vec<T>>
}

impl<T: Copy + Hash + Eq> OrbitMap<T> {
  fn new() -> OrbitMap<T> {
    OrbitMap {
      objects: HashSet::new(),
      parents: HashMap::new(),
      children: HashMap::new()
    }
  }

  fn insert(&mut self, parent: T, child: T) {
    self.objects.insert(parent);
    self.objects.insert(child);
    self.parents.insert(child, parent);

    if let Some(children) = self.children.get_mut(&parent) {
      children.push(child);
    } else {
      self.children.insert(parent, vec![child]);
    }
  }
}

fn get_input() -> std::io::Result<String> {
  let mut file = File::open("06.txt")?;
  let mut contents = String::new();
  file.read_to_string(&mut contents)?;

  Ok(contents)
}

fn parse_input(input: &str) -> OrbitMap<&str> {
  input.lines()
  .filter_map(|line| {
    let mut ids = line.split(')');

    match (ids.nth(0), ids.nth(0)) {
      (Some(parent), Some(child)) => Some((parent, child)),
      _ => None
    }
  })
  .fold(OrbitMap::new(), |mut orbit_map, (parent, child)| {
    orbit_map.insert(parent, child);
    orbit_map
  })
}

fn get_orbits<T: Copy + Hash + Eq>(orbit_map: &OrbitMap<T>, object: T) -> Vec<T> {
  let mut result = vec![];
  let mut object = object;

  while let Some(&parent) = orbit_map.parents.get(&object) {
    result.push(parent);
    object = parent;
  }

  result
}

fn bfs<T: Copy + Hash + Eq>(orbit_map: &OrbitMap<T>, start: T, target: T) -> Option<Vec<T>> {
  let mut previous_map = HashMap::new();
  previous_map.insert(start, None);

  let mut queue = VecDeque::new();
  queue.push_back(start);

  while let Some(current) = queue.pop_front() {
    if current == target {
      break;
    }

    let parent_list = [orbit_map.parents.get(&current)];
    let next_list = orbit_map.children.get(&current)
      .map(|children| children.iter().chain(
        parent_list.into_iter().filter_map(|&x| x)
      ));

    if let Some(next_list) = next_list {
      for &next in next_list {
        if !previous_map.contains_key(&next) {
          previous_map.insert(next, Some(current));
          queue.push_back(next);
        }
      }
    }
  }

  if let None = previous_map.get(&target) {
    return None;
  }

  let mut path = vec![target];

  while let Some(&Some(previous)) = previous_map.get(path.last().unwrap()) {
    path.push(previous);
  }

  path.reverse();
  Some(path)
}

fn main() {
  let input = get_input().unwrap();
  let orbit_map = parse_input(&input);

  let total_ancestors = orbit_map.objects.iter()
    .map(|&object| get_orbits(&orbit_map, object).len())
    .sum::<usize>();

  println!("Part 1: {}", total_ancestors);

  let me_orbit = orbit_map.parents["YOU"];
  let santa_orbit = orbit_map.parents["SAN"];
  let path = bfs(&orbit_map, me_orbit, santa_orbit).unwrap();

  println!("Part 2: {}", path.len() - 1);
}
