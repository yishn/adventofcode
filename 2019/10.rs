use std::fs::File;
use std::io::prelude::*;
use std::collections::HashSet;

type Point = (i32, i32);

fn get_input() -> std::io::Result<String> {
  let mut file = File::open("10.txt")?;
  let mut contents = String::new();
  file.read_to_string(&mut contents)?;

  Ok(contents)
}

fn parse_asteroid_positions(input: &str) -> HashSet<Point> {
  input.lines()
  .enumerate()
  .flat_map(|(y, line)| {
    line.chars()
    .enumerate()
    .filter_map(move |(x, c)| match c {
      '#' => Some((x as i32, y as i32)),
      _ => None
    })
  })
  .collect()
}

fn gcd(mut a: i32, mut b: i32) -> i32 {
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

fn get_discrete_angle(p1: Point, p2: Point) -> Point {
  let (dx, dy) = (p2.0 - p1.0, p2.1 - p1.1);
  let dividend = gcd(dx, dy);

  if dividend == 0 {
    (0, 0)
  } else {
    (dx / dividend, dy / dividend)
  }
}

fn get_obstacle_in_sight(obstacles: &HashSet<Point>, distance: i32, p: Point, angle: Point) -> Option<Point> {
  if angle == (0, 0) {
    return None;
  }

  for i in 1..=distance {
    let test_point = (p.0 + i * angle.0, p.1 + i * angle.1);

    if obstacles.contains(&test_point) {
      return Some(test_point);
    }
  }

  None
}

fn has_direct_sight_line(obstacles: &HashSet<Point>, p1: Point, p2: Point) -> bool {
  let angle = get_discrete_angle(p1, p2);
  let max_distance = 2 * obstacles.iter()
    .map(|&(x, y)| {
      [x.abs(), y.abs()].into_iter().max().cloned().unwrap()
    })
    .max()
    .unwrap_or(0);

  get_obstacle_in_sight(obstacles, max_distance, p1, angle)
  .map(|obstacle| obstacle == p2)
  .unwrap_or(false)
}

fn get_angles(obstacles: &HashSet<Point>, p: Point) -> Vec<Point> {
  fn clockwise_up_atan2(dx: f32, dy: f32) -> f32 {
    std::f32::consts::PI - (dx as f32).atan2(dy as f32)
  }

  let mut angles = obstacles.iter()
    .map(|&obstacle| get_discrete_angle(p, obstacle))
    .collect::<Vec<_>>();

  angles.sort_by(|&(dx1, dy1), &(dx2, dy2)| {
    clockwise_up_atan2(dx1 as f32, dy1 as f32)
    .partial_cmp(&clockwise_up_atan2(dx2 as f32, dy2 as f32))
    .unwrap_or(std::cmp::Ordering::Equal)
  });

  angles.dedup();
  angles
}

fn main() {
  let input = get_input().unwrap();
  let mut asteroid_positions = parse_asteroid_positions(&input);

  let (best_station, score) = asteroid_positions.iter()
    .map(|station| {
      let count = asteroid_positions.iter()
        .filter(|&&asteroid| has_direct_sight_line(&asteroid_positions, *station, asteroid))
        .count();

      (*station, count)
    })
    .max_by_key(|&(_, count)| count)
    .unwrap();

  println!("Part 1: {}", score);

  let angles = get_angles(&asteroid_positions, best_station);
  let mut i = 0;

  for angle in angles.into_iter().cycle() {
    if let Some(asteroid) = get_obstacle_in_sight(&asteroid_positions, 30, best_station, angle) {
      asteroid_positions.remove(&asteroid);

      i += 1;
      if i == 200 {
        let (x, y) = asteroid;
        println!("Part 2: {}", x * 100 + y);
        break;
      }
    }
  }
}
