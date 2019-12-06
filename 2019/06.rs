use std::fs::File;
use std::io::prelude::*;
use std::collections::{HashMap, HashSet, VecDeque};

#[derive(Debug)]
struct OrbitMap<'a> {
  objects: HashSet<&'a str>,
  parents: HashMap<&'a str, &'a str>,
  children: HashMap<&'a str, Vec<&'a str>>
}

impl<'a> OrbitMap<'a> {
  fn new() -> OrbitMap<'a> {
    OrbitMap {
      objects: HashSet::new(),
      parents: HashMap::new(),
      children: HashMap::new()
    }
  }

  fn insert(&mut self, parent: &'a str, child: &'a str) {
    self.objects.insert(parent);
    self.objects.insert(child);
    self.parents.insert(child, parent);

    if let Some(children) = self.children.get_mut(parent) {
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

fn parse_input<'a>(input: &'a str) -> OrbitMap<'a> {
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

fn get_ancestors<'a>(orbit_map: &'a OrbitMap, object: &'a str) -> Vec<&'a str> {
  let mut result = vec![];
  let mut object = object;

  while let Some(&parent) = orbit_map.parents.get(object) {
    result.push(parent);
    object = parent;
  }

  result
}

fn bfs<'a>(orbit_map: &'a OrbitMap, start: &'a str, target: &'a str) -> Option<Vec<&'a str>> {
  let mut previous_map = HashMap::new();
  previous_map.insert(start, None);

  let mut queue = VecDeque::new();
  queue.push_back(start);

  while let Some(current) = queue.pop_front() {
    if current == target {
      break;
    }

    let parent_list = [orbit_map.parents.get(current)];
    let next_list = orbit_map.children.get(current)
      .map(|children| children.iter().chain(
        parent_list.into_iter().filter_map(|&x| x)
      ));

    if let Some(next_list) = next_list {
      for &next in next_list {
        if !previous_map.contains_key(next) {
          previous_map.insert(next, Some(current));
          queue.push_back(next);
        }
      }
    }
  }

  if let None = previous_map.get(target) {
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
    .map(|&object| get_ancestors(&orbit_map, object).len())
    .sum::<usize>();

  println!("Part 1: {}", total_ancestors);

  let me_orbit = orbit_map.parents["YOU"];
  let santa_orbit = orbit_map.parents["SAN"];
  let path = bfs(&orbit_map, me_orbit, santa_orbit).unwrap();

  println!("Part 2: {}", path.len() - 1);
}
