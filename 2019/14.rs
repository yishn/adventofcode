use std::fs::File;
use std::{io, io::prelude::*};
use std::collections::HashMap;

type Recipe<T> = HashMap<T, u32>;
type RecipeBook<T> = HashMap<T, (Recipe<T>, u32)>;

fn get_input() -> io::Result<String> {
  let mut file = File::open("14.txt")?;
  let mut contents = String::new();
  file.read_to_string(&mut contents)?;

  Ok(contents)
}

fn parse_input(input: &str) -> RecipeBook<&str> {
  fn parse_ingredient(input: &str) -> Option<(&str, u32)> {
    let mut tokens = input.trim().split(" ");

    match (tokens.next(), tokens.next()) {
      (Some(amount), Some(id)) => {
        amount.parse::<u32>().ok()
        .map(|amount| (id, amount))
      },
      _ => None
    }
  }

  input.lines()
  .map(|line| line.split(" => "))
  .filter_map(|mut tokens| match (tokens.next(), tokens.next()) {
    (Some(ingredients), Some(outcome)) => {
      Some((ingredients.split(", "), outcome))
    },
    _ => None
  })
  .map(|(ingredients, outcome)| (ingredients.filter_map(parse_ingredient), parse_ingredient(outcome)))
  .filter_map(|(ingredients, outcome)| outcome.and_then(|outcome| {
    let ingredients = ingredients.collect::<HashMap<_, _>>();

    if ingredients.len() == 0 {
      return None;
    }

    Some((outcome.0, (ingredients, outcome.1)))
  }))
  .collect()
}

fn main() {
  let input = get_input().unwrap();
  let recipes = parse_input(&input);

  println!("{:?}", recipes);
}
