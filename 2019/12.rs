use std::fs::File;
use std::io::prelude::*;
use std::fmt;

type Vector3 = (i32, i32, i32);

#[derive(Clone)]
struct MoonState {
  position: Vector3,
  velocity: Vector3
}

impl fmt::Debug for MoonState {
  fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
    write!(
      f, "pos=<x={}, y={}, z={}>, vel=<x={}, y={}, z={}>",
      self.position.0,
      self.position.1,
      self.position.2,
      self.velocity.0,
      self.velocity.1,
      self.velocity.2
    )
  }
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

fn gcd(a: i64, b: i64) -> i64 {
  let (mut a, mut b) = (a, b);
  let mut h;

  if a == 0 {
    return b.abs();
  } else if b == 0 {
    return a.abs();
  }

  loop {
    h = a % b;
    a = b;
    b = h;

    if b == 0 {
      break a.abs();
    }
  }
}

fn lcm(a: i64, b: i64) -> i64 {
  if a == 0 && b == 0 {
    0
  } else {
    a.abs() / gcd(a, b) * b.abs()
  }
}

fn multiple_lcm(numbers: &[i64]) -> Option<i64> {
  numbers.iter()
  .fold(None, |acc, &x| match acc {
    None => Some(x),
    Some(acc) => Some(lcm(acc, x))
  })
}

fn calc_acceleration(pos1: Vector3, pos2: Vector3) -> Vector3 {
  fn get_sign(x: i32, y: i32) -> i32 {
    if x < y { 1 }
    else if x > y { -1 }
    else { 0 }
  }

  (
    get_sign(pos1.0, pos2.0),
    get_sign(pos1.1, pos2.1),
    get_sign(pos1.2, pos2.2)
  )
}

fn simulate_tick(moons: &mut Vec<MoonState>) {
  // Apply gravity

  let n = moons.len();
  let pairs = (0..n)
    .flat_map(|x| {
      (0..n)
      .map(move |y| (x, y))
    })
    .filter(|&(x, y)| x != y);

  for (i, j) in pairs {
    let moon1 = &moons[i];
    let moon2 = &moons[j];
    let acceleration = calc_acceleration(moon1.position, moon2.position);

    let moon1 = &mut moons[i];

    moon1.velocity = (
      moon1.velocity.0 + acceleration.0,
      moon1.velocity.1 + acceleration.1,
      moon1.velocity.2 + acceleration.2
    );
  }

  // Apply velocity

  for moon in moons.iter_mut() {
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

  let mut moons = initial_moons.clone();
  let (mut xf, mut yf, mut zf) = (false, false, false);
  let (mut xp, mut yp, mut zp) = (0, 0, 0);

  fn count_var<F, T>(
    counter: &mut i64,
    finished: &mut bool,
    moons: &Vec<MoonState>,
    compare: &Vec<MoonState>,
    predicate: F
  )
  where
    T: PartialEq,
    F: Fn(&MoonState) -> T
  {
    if !*finished {
      *counter += 1;

      let component_equals = moons.iter().map(&predicate)
        .zip(compare.iter().map(&predicate))
        .all(|(x, y)| x == y);

      if component_equals {
        *finished = true;
      }
    }
  }

  loop {
    simulate_tick(&mut moons);

    count_var(&mut xp, &mut xf, &moons, &initial_moons, |moon| [moon.position.0, moon.velocity.0]);
    count_var(&mut yp, &mut yf, &moons, &initial_moons, |moon| [moon.position.1, moon.velocity.1]);
    count_var(&mut zp, &mut zf, &moons, &initial_moons, |moon| [moon.position.2, moon.velocity.2]);

    if xf && yf && zf {
      break;
    }
  }

  println!("Part 2: {}", multiple_lcm(&[xp, yp, zp]).unwrap());
}
