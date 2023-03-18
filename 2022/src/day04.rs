use std::str::FromStr;

static INPUT: &'static str = include_str!("./day04.txt");

#[derive(Debug)]
struct Range(u8, u8);

impl Range {
  pub fn contains(&self, other: &Range) -> bool {
    self.0 <= other.0 && self.1 >= other.1
  }

  pub fn overlaps(&self, other: &Range) -> bool {
    self.0 <= other.0 && self.1 >= other.0
      || self.0 >= other.0 && other.1 >= self.0
  }
}

impl FromStr for Range {
  type Err = ();

  fn from_str(s: &str) -> Result<Self, Self::Err> {
    let mut tokens = s.split("-");
    let start = tokens.next();
    let end = tokens.next();

    match (start, end) {
      (Some(start), Some(end)) => Ok(Range(
        start.parse::<u8>().map_err(|_| ())?,
        end.parse::<u8>().map_err(|_| ())?,
      )),
      _ => Err(()),
    }
  }
}

fn parse_input(input: &str) -> Vec<(Range, Range)> {
  input
    .lines()
    .filter_map(|line| {
      let mut tokens = line.trim().split(",");
      let first_range = tokens.next();
      let second_range = tokens.next();

      match (first_range, second_range) {
        (Some(first_range), Some(second_range)) => {
          Some((first_range, second_range))
        }
        _ => None,
      }
    })
    .filter_map(|(first_range, second_range)| {
      match (first_range.parse::<Range>(), second_range.parse::<Range>()) {
        (Ok(first), Ok(second)) => Some((first, second)),
        _ => None,
      }
    })
    .collect()
}

#[test]
fn part1() {
  let schedule = parse_input(INPUT);

  println!(
    "{}",
    schedule
      .iter()
      .filter(
        |(range1, range2)| range1.contains(range2) || range2.contains(range1)
      )
      .count()
  );
}

#[test]
fn part2() {
  let schedule = parse_input(INPUT);

  println!(
    "{}",
    schedule
      .iter()
      .filter(|(range1, range2)| range1.overlaps(range2))
      .count()
  );
}
