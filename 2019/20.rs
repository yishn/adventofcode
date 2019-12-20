use std::fs::File;
use std::io::prelude::*;
use std::hash::Hash;
use std::collections::HashMap;

mod graph;
use graph::Graph;

#[derive(Debug, Copy, Clone, PartialEq)]
enum PortalType {
  Inner,
  Outer
}

#[derive(Debug, Copy, Clone)]
enum Tile<P> {
  Wall,
  Passage,
  Portal(P, PortalType)
}

type Position = (usize, usize);

#[derive(Debug)]
struct Labyrinth<N: Hash + Eq, P: Hash + Eq> {
  tiles: HashMap<N, Tile<P>>,
  portals: HashMap<P, (N, N)>,
  entrance: Option<N>,
  goal: Option<N>,
}

impl<P: Hash + Eq + Clone> Graph<Position> for Labyrinth<Position, P> {
  fn get_neighbors(&self, (x, y): Position) -> Vec<Position> {
    let portal = match self.tiles.get(&(x, y)) {
      Some(Tile::Portal(p, _)) => Some(p.clone()),
      _ => None
    };

    let mut result = vec![(x - 1, y), (x + 1, y), (x, y - 1), (x, y + 1)].into_iter()
      .filter(|&(x, y)| x > 0 && y > 0)
      .filter(|pos| match self.tiles.get(pos) {
        Some(Tile::Passage) | Some(Tile::Portal(_, _)) => true,
        _ => false
      })
      .collect::<Vec<_>>();

    if let Some(p) = portal {
      let (pos1, pos2) = self.portals.get(&p).cloned().unwrap();

      if pos1 == (x, y) {
        result.push(pos2);
      } else {
        result.push(pos1);
      }
    }

    result
  }
}

impl<P: Hash + Eq + Clone> Graph<(Position, usize)> for Labyrinth<Position, P> {
  fn get_neighbors(&self, ((x, y), level): (Position, usize)) -> Vec<(Position, usize)> {
    let (portal, portal_type) = match self.tiles.get(&(x, y)) {
      Some(Tile::Portal(p, t)) => (Some(p.clone()), Some(t.clone())),
      _ => (None, None)
    };

    let mut result = vec![(x - 1, y), (x + 1, y), (x, y - 1), (x, y + 1)].into_iter()
      .filter(|&(x, y)| x > 0 && y > 0)
      .filter(|pos| match self.tiles.get(pos) {
        Some(Tile::Passage) | Some(Tile::Portal(_, _)) => true,
        _ => false
      })
      .map(|pos| (pos, level))
      .collect::<Vec<_>>();

    if let (Some(p), Some(portal_type)) = (portal, portal_type) {
      if level > 0 && portal_type == PortalType::Outer || portal_type == PortalType::Inner {
        let (pos1, pos2) = self.portals.get(&p).cloned().unwrap();

        result.push((
          if pos1 == (x, y) { pos2 } else { pos1 },
          if portal_type == PortalType::Outer { level - 1 } else { level + 1 }
        ));
      }
    }

    result
  }
}

fn get_input() -> std::io::Result<String> {
  let mut file = File::open("20.txt")?;
  let mut contents = String::new();
  file.read_to_string(&mut contents)?;

  Ok(contents)
}

fn parse_labyrinth(input: &str) -> Labyrinth<Position, String> {
  let height = input.lines().count();
  let width = input.lines().next().map(|row| row.len()).unwrap_or(0);
  let mut character_map = HashMap::new();
  let mut tiles = HashMap::new();
  let mut portals = HashMap::<String, Vec<Position>>::new();
  let mut entrance = None;
  let mut goal = None;

  for (y, line) in input.lines().enumerate() {
    for (x, c) in line.chars().enumerate() {
      character_map.insert((x, y), c);
    }
  }

  for (&(x, y), &c) in character_map.iter() {
    tiles.insert((x, y), match c {
      '#' => Tile::Wall,
      '.' => {
        let get_label_positions = |(x, y), d| (
          (x - 3 + d, y), // Left
          (x + d, y),     // Right
          (x, y - 3 + d), // Top
          (x, y + d)      // Bottom
        );

        let mut label_char_iter = [1, 2].into_iter().cloned()
          .map(|d| get_label_positions((x as isize, y as isize), d));

        let (portal_type, mut label) = match (label_char_iter.next(), label_char_iter.next()) {
          (Some((l11, l21, l31, l41)), Some((l12, l22, l32, l42))) => {
            [(l11, l12), (l21, l22), (l31, l32), (l41, l42)]
            .into_iter()
            .cloned()
            .map(|((a, b), (c, d))| ((a as usize, b as usize), (c as usize, d as usize)))
            .enumerate()
            .filter_map(|(i, (pos1, pos2))| match (character_map.get(&pos1), character_map.get(&pos2)) {
              (Some(&c1), Some(&c2)) => Some((i, (c1, c2))),
              _ => None
            })
            .filter(|&(_, (c1, c2))| c1.is_ascii_alphabetic() && c2.is_ascii_alphabetic())
            .map(|(i, (c1, c2))| {
              let mut label = String::new();
              label.push(c1);
              label.push(c2);

              let portal_type = match [
                (x, width, PortalType::Outer, PortalType::Inner),  // Left
                (x, width, PortalType::Inner, PortalType::Outer),  // Right
                (y, height, PortalType::Outer, PortalType::Inner), // Top
                (y, height, PortalType::Inner, PortalType::Outer)  // Bottom
              ][i] {
                (a, b, x, y) => if a < b / 2 { x } else { y }
              };

              (Some(portal_type), Some(label))
            })
            .next()
            .unwrap_or((None, None))
          },
          _ => (None, None)
        };

        match label.as_ref().map(|s| &s[..]) {
          Some("ZZ") => {
            label = None;
            goal = Some((x, y));
          },
          Some("AA") => {
            label = None;
            entrance = Some((x, y));
          },
          _ => {}
        }

        if let (Some(portal_type), Some(label)) = (portal_type, label) {
          match portals.get_mut(&label) {
            Some(positions) => {
              positions.push((x, y));
            },
            None => {
              portals.insert(label.clone(), vec![(x, y)]);
            }
          }

          Tile::Portal(label, portal_type)
        } else {
          Tile::Passage
        }
      }
      _ => continue
    });
  }

  let portals = portals.into_iter()
    .filter(|(_, positions)| positions.len() == 2)
    .map(|(key, positions)| (key, (positions[0], positions[1])))
    .collect();

  Labyrinth {
    tiles,
    portals,
    entrance,
    goal
  }
}

fn main() {
  let input = get_input().unwrap();
  let labyrinth = parse_labyrinth(&input);
  let entrance = labyrinth.entrance.unwrap();
  let goal = labyrinth.goal.unwrap();

  let path = labyrinth.bfs(entrance).construct_path(goal).unwrap();
  println!("Part 1: {}", path.len() - 1);

  let path = labyrinth.bfs((entrance, 0)).construct_path((goal, 0)).unwrap();
  println!("Part 2: {}", path.len() - 1);
}
