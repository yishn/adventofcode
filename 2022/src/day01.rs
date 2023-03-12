static INPUT: &'static str = include_str!("./day01.txt");

#[derive(Debug)]
struct Elf {
  food_items: Vec<u64>,
}

impl Elf {
  pub fn sum_calories(&self) -> u64 {
    self.food_items.iter().sum()
  }
}

fn parse_input(input: &str) -> Vec<Elf> {
  input
    .split("\n\n")
    .map(|calories| Elf {
      food_items: calories
        .lines()
        .filter_map(|line| line.parse::<u64>().ok())
        .collect(),
    })
    .collect()
}

#[test]
fn part1() {
  let elves = parse_input(INPUT);

  println!(
    "{}",
    elves.iter().map(|elf| elf.sum_calories()).max().unwrap()
  );
}

#[test]
fn part2() {
  let mut elves = parse_input(INPUT);

  elves.sort_by_cached_key(|elf| elf.sum_calories());
  elves.reverse();

  println!(
    "{}",
    elves
      .iter()
      .take(3)
      .map(|elf| elf.sum_calories())
      .sum::<u64>()
  );
}
