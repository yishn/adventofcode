use std::env;
use std::fs::File;
use std::iter;
use std::io;
use std::io::prelude::*;
use std::collections::HashMap;

mod intcode;
mod graph;

use intcode::*;
use graph::Graph;

#[derive(Debug)]
struct ShipMap {
  rooms: Vec<String>,
  passages: HashMap<usize, HashMap<usize, String>>,
  items: HashMap<usize, Vec<String>>
}

impl ShipMap {
  fn new(rooms: &[&str]) -> ShipMap {
    ShipMap {
      rooms: rooms.iter().map(|&x| x.to_owned()).collect(),
      passages: HashMap::new(),
      items: HashMap::new()
    }
  }

  fn get_room_id(&self, room: &str) -> Option<usize> {
    self.rooms.iter().position(|r| r == room)
  }

  fn insert_passage(&mut self, room1: &str, direction: &str, room2: &str) {
    let id1 = self.get_room_id(room1);
    let id2 = self.get_room_id(room2);

    let opposite = match direction {
      "north" => "south",
      "west" => "east",
      "south" => "north",
      "east" => "west",
      _ => return
    };

    if let (Some(id1), Some(id2)) = (id1, id2) {
      if let Some(value) = self.passages.get_mut(&id1) {
        value.insert(id2, direction.to_owned());
      } else {
        self.passages.insert(id1, iter::once((id2, direction.to_owned())).collect());
      }

      if let Some(value) = self.passages.get_mut(&id2) {
        value.insert(id1, opposite.to_owned());
      } else {
        self.passages.insert(id2, iter::once((id1, opposite.to_owned())).collect());
      }
    }
  }
}

impl Graph<usize> for ShipMap {
  fn get_neighbors(&self, room_id: usize) -> Vec<usize> {
    self.passages.get(&room_id)
    .map(|passages| passages.keys().cloned().collect::<Vec<_>>())
    .unwrap_or_else(|| vec![])
  }
}

fn get_input() -> std::io::Result<String> {
  let mut file = File::open("25.txt")?;
  let mut contents = String::new();
  file.read_to_string(&mut contents)?;

  Ok(contents)
}

fn get_ship() -> ShipMap {
  // Returns premapped ship

  let mut ship = ShipMap::new(&[
    "Hull Breach",
    "Engineering",
    "Hallway",
    "Observatory",
    "Storage",
    "Security",
    "Gift Wrapping",
    "Arcade",
    "Crew",
    "Kitchen",
    "Passages",
    "Navigation",
    "Corridor",
    "Stables",
    "Sick Bay",
    "Warp Drive",
    "Science Lab"
  ]);

  ship.items = [
    ("Storage", vec!["space heater"]),
    ("Gift Wrapping", vec!["antenna"]),
    ("Crew", vec!["fixed point"]),
    ("Kitchen", vec!["asterisk"]),
    ("Passages", vec!["festive hat"]),
    ("Corridor", vec!["jam"]),
    ("Stables", vec!["easter egg"]),
    ("Sick Bay", vec!["tambourine"])
  ].into_iter()
    .filter_map(|(room, items)| (
      ship.get_room_id(room)
      .map(|id| (id, items.iter().map(|&x| x.to_owned()).collect::<Vec<_>>()))
    ))
    .collect();

  ship.insert_passage("Hull Breach", "south", "Engineering");
  ship.insert_passage("Arcade", "north", "Engineering");
  ship.insert_passage("Arcade", "south", "Crew");
  ship.insert_passage("Arcade", "west", "Science Lab");
  ship.insert_passage("Warp Drive", "south", "Science Lab");
  ship.insert_passage("Warp Drive", "north", "Sick Bay");
  ship.insert_passage("Kitchen", "east", "Crew");
  ship.insert_passage("Passages", "north", "Crew");
  ship.insert_passage("Passages", "west", "Navigation");
  ship.insert_passage("Corridor", "east", "Navigation");
  ship.insert_passage("Corridor", "south", "Stables");
  ship.insert_passage("Hallway", "east", "Engineering");
  ship.insert_passage("Hallway", "south", "Gift Wrapping");
  ship.insert_passage("Hallway", "west", "Observatory");
  ship.insert_passage("Storage", "east", "Observatory");
  ship.insert_passage("Storage", "west", "Security");

  ship
}

fn move_droid(state: &mut ProgramState, ship: &ShipMap, room: &mut String, target: &str) -> bool {
  let id = ship.get_room_id(room);
  let target_id = ship.get_room_id(target);

  if let (Some(mut id), Some(target_id)) = (id, target_id) {
    let path = ship.bfs(id).construct_path(target_id).unwrap_or_else(|| vec![]);

    for next_room_id in path.into_iter().skip(1) {
      let direction = ship.passages.get(&id)
        .and_then(|neighbored_rooms| neighbored_rooms.get(&next_room_id));

      if let Some(direction) = direction {
        let input = direction.to_owned() + "\n";
        let (output, _) = run_ascii_program_with_input(state, &input);

        id = next_room_id;
        *room = ship.rooms[next_room_id].clone();
      } else {
        break;
      }
    }
  }

  target == room
}

fn get_subsets<T: Clone>(set: &[T]) -> Vec<Vec<T>> {
  set.first()
  .map(|first| {
    get_subsets(&set[1..]).into_iter()
    .chain(
      get_subsets(&set[1..]).into_iter()
      .map(|mut subset| {
        subset.push(first.clone());
        subset
      })
    )
    .collect()
  })
  .unwrap_or_else(|| vec![vec![]])
}

fn main() {
  let input = get_input().unwrap();
  let program = input.split(',')
    .filter_map(|x| x.trim().parse::<i64>().ok())
    .collect::<Vec<_>>();

  let mut state = ProgramState::new(program.clone());

  if env::args().any(|s| s == "--print") {
    let stdin = io::stdin();

    for line in iter::once(Ok("".to_owned())).chain(stdin.lock().lines()) {
      let mut line = line.unwrap();

      if line.len() > 0 {
        line.push('\n');
      }

      let (output, result) = run_ascii_program_with_input(&mut state, &line);
      print!("{}", output);

      if let ProgramResult::Halt = result {
        break;
      }
    }
  }

  let ship = get_ship();

  // First, collect all collectable items

  let mut all_items = vec![];
  let mut room = "Hull Breach".to_owned();
  let dfs_iter = ship.dfs(ship.get_room_id(&room).unwrap());

  run_ascii_program_with_input(&mut state, "");

  for next_room_id in dfs_iter {
    let next_room = &ship.rooms[next_room_id];
    move_droid(&mut state, &ship, &mut room, next_room);

    let items = ship.items.get(&next_room_id);

    if let Some(items) = items {
      for item in items {
        let input = "take ".to_owned() + item + "\n";
        run_ascii_program_with_input(&mut state, &input);
        all_items.push(item.clone());
      }
    }
  }

  // Move to Security

  move_droid(&mut state, &ship, &mut room, "Security");

  // Try all combinations of items

  let subsets = get_subsets(&all_items);

  for subset in &subsets {
    // Drop items in the subset

    for item in subset {
      let input = "drop ".to_owned() + item + "\n";
      run_ascii_program_with_input(&mut state, &input);
    }

    // Try to get past security

    let (output, _) = run_ascii_program_with_input(&mut state, "west\n");

    if output.contains("hello") {
      let line = output.lines()
        .find(|line| line.contains("hello"))
        .unwrap();

      println!("Part 1: {}", line);
      break;
    }

    // Retake dropped items

    for item in subset {
      let input = "take ".to_owned() + item + "\n";
      run_ascii_program_with_input(&mut state, &input);
    }
  }
}
