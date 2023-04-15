use std::{collections::HashMap, convert::Infallible, str::FromStr};

static INPUT: &'static str = include_str!("./day08.txt");

type Position = (usize, usize);

#[derive(Debug)]
struct Forest {
  trees: HashMap<Position, u8>,
}

impl FromStr for Forest {
  type Err = Infallible;

  fn from_str(s: &str) -> Result<Self, Self::Err> {
    Ok(Self {
      trees: s
        .lines()
        .enumerate()
        .flat_map(|(y, line)| {
          line.chars().enumerate().filter_map(move |(x, c)| {
            c.to_string()
              .parse::<u8>()
              .ok()
              .map(|height| ((x, y), height))
          })
        })
        .collect::<HashMap<_, _>>(),
    })
  }
}

impl Forest {
  pub fn is_visible(&self, (x, y): Position) -> bool {
    let height = self.trees.get(&(x, y)).copied().unwrap_or(0);

    let check_height = |iter: &mut dyn Iterator<Item = Position>| -> bool {
      iter
        .filter_map(|pos| self.trees.get(&pos))
        .all(|&h| h < height)
    };

    check_height(&mut (0..x).map(|x| (x, y)))
      || check_height(&mut (0..y).map(|y| (x, y)))
      || check_height(
        &mut (x + 1..)
          .map(|x| (x, y))
          .take_while(|pos| self.trees.contains_key(pos)),
      )
      || check_height(
        &mut (y + 1..)
          .map(|y| (x, y))
          .take_while(|pos| self.trees.contains_key(pos)),
      )
  }

  pub fn scenic_score(&self, (x, y): Position) -> usize {
    let height = self.trees.get(&(x, y)).copied().unwrap_or(0);

    let count_trees = |iter: &mut dyn Iterator<Item = Position>| -> usize {
      let heights = iter
        .filter_map(|pos| self.trees.get(&pos).copied())
        .collect::<Vec<_>>();
      let mut result = heights.iter().take_while(|&&h| h < height).count();

      if heights.get(result).is_some() {
        result += 1;
      }

      result
    };

    let left_trees_count = count_trees(&mut (0..x).rev().map(|x| (x, y)));
    let top_trees_count = count_trees(&mut (0..y).rev().map(|y| (x, y)));
    let right_trees_count = count_trees(
      &mut (x + 1..)
        .map(|x| (x, y))
        .take_while(|pos| self.trees.contains_key(pos)),
    );
    let bottom_trees_count = count_trees(
      &mut (y + 1..)
        .map(|y| (x, y))
        .take_while(|pos| self.trees.contains_key(pos)),
    );

    left_trees_count * top_trees_count * right_trees_count * bottom_trees_count
  }
}

#[test]
fn part1() {
  let forest = INPUT.parse::<Forest>().unwrap();

  println!(
    "{}",
    forest
      .trees
      .keys()
      .filter(|&&pos| forest.is_visible(pos))
      .count()
  );
}

#[test]
fn part2() {
  let forest = INPUT.parse::<Forest>().unwrap();

  println!(
    "{}",
    forest
      .trees
      .keys()
      .map(|&pos| forest.scenic_score(pos))
      .max()
      .unwrap()
  );
}
