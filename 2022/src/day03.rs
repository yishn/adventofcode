use std::collections::HashSet;

static INPUT: &'static str = include_str!("./day03.txt");

#[derive(Debug, PartialEq, Eq, Hash, Clone)]
struct Item(char);

impl Item {
  pub fn from_char(char: char) -> Result<Item, char> {
    if char.is_ascii_alphabetic() {
      Ok(Item(char))
    } else {
      Err(char)
    }
  }

  pub fn priority(&self) -> u64 {
    if self.0.is_ascii_lowercase() {
      self.0 as u64 - 0x60
    } else if self.0.is_ascii_uppercase() {
      self.0 as u64 - 0x41 + 27
    } else {
      unreachable!()
    }
  }
}

#[derive(Debug)]
struct Rucksack {
  compartment1: Vec<Item>,
  compartment2: Vec<Item>,
}

impl Rucksack {
  pub fn intersect_compartments(&self) -> HashSet<Item> {
    self
      .compartment1
      .iter()
      .collect::<HashSet<_>>()
      .intersection(&self.compartment2.iter().collect::<HashSet<_>>())
      .cloned()
      .cloned()
      .collect::<HashSet<_>>()
  }

  pub fn iter(&self) -> impl Iterator<Item = &Item> + '_ {
    self.compartment1.iter().chain(self.compartment2.iter())
  }
}

fn parse_input(input: &str) -> Vec<Rucksack> {
  input
    .trim()
    .lines()
    .map(|line| {
      let characters = line
        .trim()
        .chars()
        .filter_map(|char| Item::from_char(char).ok())
        .collect::<Vec<_>>();
      let compartment_size = characters.len() / 2;

      Rucksack {
        compartment1: characters[..compartment_size].to_vec(),
        compartment2: characters[compartment_size..].to_vec(),
      }
    })
    .collect()
}

#[test]
fn part1() {
  let rucksacks = parse_input(INPUT);

  println!(
    "{}",
    rucksacks
      .iter()
      .map(|rucksack| rucksack
        .intersect_compartments()
        .iter()
        .map(|item| item.priority())
        .sum::<u64>())
      .sum::<u64>()
  );
}

#[test]
fn part2() {
  let rucksacks = parse_input(INPUT);

  let badge_priority_sum = (0..rucksacks.len())
    .step_by(3)
    .map(|i| &rucksacks[i..i + 3])
    .filter_map(|rucksacks| {
      rucksacks.iter().fold(None::<HashSet<_>>, |acc, rucksack| {
        acc.map_or_else(
          || Some(rucksack.iter().collect()),
          |acc| {
            Some(
              acc
                .intersection(&rucksack.iter().collect())
                .copied()
                .collect(),
            )
          },
        )
      })
    })
    .flat_map(|intersection| intersection.into_iter())
    .map(|item| item.priority())
    .sum::<u64>();

  println!("{}", badge_priority_sum);
}
