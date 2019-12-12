use std::fs::File;
use std::io::prelude::*;

type Vector3 = (i32, i32, i32);

#[derive(Debug, Clone, Eq, PartialEq)]
struct MoonState {
  position: Vector3,
  velocity: Vector3
}

impl MoonState {
  fn potential_energy(&self) -> i32 {
    norm(self.position)
  }

  fn kinetic_enery(&self) -> i32 {
    norm(self.velocity)
  }

  fn total_energy(&self) -> i32 {
    self.potential_energy() * self.kinetic_enery()
  }
}

fn get_input() -> std::io::Result<String> {
  let mut file = File::open("12.txt")?;
  let mut contents = String::new();
  file.read_to_string(&mut contents)?;

  Ok(contents)
}

fn parse_input(input: &str) -> Vec<MoonState> {
  input.lines()
  .filter_map(|line| line.get(1..line.len() - 1))
  .filter_map(|line| {
    let mut coords = line.split(", ")
      .filter_map(|token| {
        match token.split('=').nth(1) {
          Some(x) => x.parse::<i32>().ok(),
          _ => None
        }
      });

    match (coords.nth(0), coords.nth(0), coords.nth(0)) {
      (Some(x), Some(y), Some(z)) => Some((x, y, z)),
      _ => None
    }
  })
  .map(|position| MoonState {
    position,
    velocity: (0, 0, 0)
  })
  .collect()
}

fn norm((x, y, z): Vector3) -> i32 {
  x.abs() + y.abs() + z.abs()
}

fn simulate_tick(moons: &mut Vec<MoonState>) {
  // Apply gravity

  fn calc_acceleration(x: i32, y: i32) -> i32 {
    if x < y { 1 }
    else if x > y { -1 }
    else { 0 }
  }

  let n = moons.len();
  let pairs = (0..n)
    .flat_map(|x| {
      (0..n)
      .map(move |y| (x, y))
    })
    .filter(|&(x, y)| x < y);

  for (i, j) in pairs {
    let moon1 = &moons[i];
    let moon2 = &moons[j];

    let acceleration = (
      calc_acceleration(moon1.position.0, moon2.position.0),
      calc_acceleration(moon1.position.1, moon2.position.1),
      calc_acceleration(moon1.position.2, moon2.position.2)
    );

    let moon1 = &mut moons[i];

    moon1.velocity = (
      moon1.velocity.0 + acceleration.0,
      moon1.velocity.1 + acceleration.1,
      moon1.velocity.2 + acceleration.2
    );

    let moon2 = &mut moons[j];

    moon2.velocity = (
      moon2.velocity.0 - acceleration.0,
      moon2.velocity.1 - acceleration.1,
      moon2.velocity.2 - acceleration.2
    );
  }

  // Apply velocity

  for moon in moons {
    moon.position = (
      moon.position.0 + moon.velocity.0,
      moon.position.1 + moon.velocity.1,
      moon.position.2 + moon.velocity.2
    );
  }
}

fn main() {
  let input = get_input().unwrap();
  let initial_moons = parse_input(&input);
  let mut moons = initial_moons.clone();

  for _ in 0..1000 {
    simulate_tick(&mut moons);
  }

  let total_energy = moons.iter()
    .map(|moon| moon.total_energy())
    .sum::<i32>();

  println!("Part 1: {}", total_energy);
}
