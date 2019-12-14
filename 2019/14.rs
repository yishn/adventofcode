use std::fs::File;
use std::{io, io::prelude::*};
use std::hash::Hash;
use std::fmt::Debug;
use std::collections::HashMap;

type Recipe<T> = Vec<(T, u32)>;
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
    let ingredients = ingredients.collect::<Recipe<_>>();

    if ingredients.len() == 0 {
      return None;
    }

    Some((outcome.0, (ingredients, outcome.1)))
  }))
  .collect()
}

fn get_base_ingredient_amount<T>(recipes: &RecipeBook<T>, ingredient: T, amount: u32, residues: &mut HashMap<T, u32>) -> u32
where T: Eq + Clone + Copy + Debug + Hash {
  let mut amount = amount;

  if let Some(residue) = residues.get_mut(&ingredient) {
    if *residue >= amount {
      amount = 0;
      *residue -= amount;
    } else {
      amount -= *residue;
      *residue = 0;
    }
  }

  if amount == 0 {
    return 0;
  }

  let result = match recipes.get(&ingredient) {
    None => amount,
    Some((subrecipe, yield_amount)) => {
      let n = ((amount as f64) / (*yield_amount as f64)).ceil() as u32;
      let residue = yield_amount * n - amount;

      let result = subrecipe.iter()
        .map(|&(ingredient, amount)| {
          get_base_ingredient_amount(recipes, ingredient, amount * n, residues)
        })
        .sum();

      if residue > 0 {
        let new_residue = residues.get(&ingredient).cloned().unwrap_or(0) + residue;
        residues.insert(ingredient, new_residue);
      }

      result
    }
  };

  result
}

fn main() {
  let input = get_input().unwrap();
  let recipes = parse_input(&input);

  let x = get_base_ingredient_amount(&recipes, "FUEL", 1, &mut HashMap::new());

  println!("{:?}", x);
}
