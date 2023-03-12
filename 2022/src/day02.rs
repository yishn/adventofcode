use std::str::FromStr;

static INPUT: &'static str = include_str!("./day02.txt");

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
enum Choice {
  Rock = 0,
  Paper = 1,
  Scissors = 2,
}

impl FromStr for Choice {
  type Err = ();

  fn from_str(s: &str) -> Result<Self, Self::Err> {
    if s == "A" || s == "X" {
      Ok(Choice::Rock)
    } else if s == "B" || s == "Y" {
      Ok(Choice::Paper)
    } else if s == "C" || s == "Z" {
      Ok(Choice::Scissors)
    } else {
      Err(())
    }
  }
}

impl Choice {
  fn from_outcome(outcome: Outcome, opponent_choice: Choice) -> Choice {
    let offset = match outcome {
      Outcome::Win => 1,
      Outcome::Loss => 2,
      Outcome::Draw => 0,
    };

    match (opponent_choice as u8 + offset) % 3 {
      0 => Choice::Rock,
      1 => Choice::Paper,
      2 => Choice::Scissors,
      _ => unreachable!(),
    }
  }

  fn score(&self) -> u64 {
    *self as u64 + 1
  }

  fn fight(&self, opponent_choice: Choice) -> Outcome {
    if (*self as u8 + 1) % 3 == opponent_choice as u8 {
      Outcome::Loss
    } else if (*self as u8 + 2) % 3 == opponent_choice as u8 {
      Outcome::Win
    } else {
      Outcome::Draw
    }
  }
}

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
enum Outcome {
  Win,
  Loss,
  Draw,
}

impl FromStr for Outcome {
  type Err = ();

  fn from_str(s: &str) -> Result<Self, Self::Err> {
    if s == "X" {
      Ok(Outcome::Loss)
    } else if s == "Y" {
      Ok(Outcome::Draw)
    } else if s == "Z" {
      Ok(Outcome::Win)
    } else {
      Err(())
    }
  }
}

impl Outcome {
  fn score(&self) -> u64 {
    match self {
      Outcome::Win => 6,
      Outcome::Loss => 0,
      Outcome::Draw => 3,
    }
  }
}

fn parse_input_part1(input: &str) -> Vec<(Choice, Choice)> {
  input
    .lines()
    .filter_map(|line| {
      let mut tokens = line.split_whitespace();
      let opponent_token = tokens.next();
      let player_token = tokens.next();

      match (opponent_token, player_token) {
        (Some(opponent_token), Some(player_token)) => match (
          opponent_token.parse::<Choice>(),
          player_token.parse::<Choice>(),
        ) {
          (Ok(opponent_choice), Ok(player_choice)) => {
            Some((opponent_choice, player_choice))
          }
          _ => None,
        },
        _ => None,
      }
    })
    .collect()
}

fn parse_input_part2(input: &str) -> Vec<(Choice, Outcome)> {
  input
    .lines()
    .filter_map(|line| {
      let mut tokens = line.split_whitespace();
      let opponent_token = tokens.next();
      let outcome_token = tokens.next();

      match (opponent_token, outcome_token) {
        (Some(opponent_token), Some(outcome_token)) => match (
          opponent_token.parse::<Choice>(),
          outcome_token.parse::<Outcome>(),
        ) {
          (Ok(opponent_choice), Ok(outcome)) => {
            Some((opponent_choice, outcome))
          }
          _ => None,
        },
        _ => None,
      }
    })
    .collect()
}

#[test]
pub fn part1() {
  let input = parse_input_part1(INPUT);

  println!(
    "{}",
    input
      .iter()
      .copied()
      .map(|(opponent_choice, player_choice)| {
        player_choice.fight(opponent_choice).score() + player_choice.score()
      })
      .sum::<u64>()
  )
}

#[test]
pub fn part2() {
  let input = parse_input_part2(INPUT);

  println!(
    "{}",
    input
      .iter()
      .copied()
      .map(|(opponent_choice, desired_outcome)| {
        let player_choice =
          Choice::from_outcome(desired_outcome, opponent_choice);
        desired_outcome.score() + player_choice.score()
      })
      .sum::<u64>()
  )
}
