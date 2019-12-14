use std::fs::File;
use std::{io, io::prelude::*};
use std::hash::Hash;
use std::fmt::Debug;
use std::collections::HashMap;

type Recipe<T> = HashMap<T, u64>;
type RecipeBook<T> = HashMap<T, (Recipe<T>, u64)>;

fn get_input() -> io::Result<String> {
  let mut file = File::open("14.txt")?;
  let mut contents = String::new();
  file.read_to_string(&mut contents)?;

  Ok(contents)
}

fn parse_input(input: &str) -> RecipeBook<&str> {
  fn parse_ingredient(input: &str) -> Option<(&str, u64)> {
    let mut tokens = input.trim().split(" ");

    match (tokens.next(), tokens.next()) {
      (Some(amount), Some(id)) => {
        amount.parse::<u64>().ok()
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

fn get_base_ingredient_amount<T>(recipes: &RecipeBook<T>, ingredient: T, amount: u64, residues: &mut Recipe<T>) -> u64
where T: Eq + Clone + Copy + Debug + Hash {
  let mut amount = amount;

  if let Some(residue) = residues.get_mut(&ingredient) {
    if *residue >= amount {
      *residue -= amount;
      amount = 0;
    } else {
      amount -= *residue;
      *residue = 0;
    }
  }

  if amount == 0 {
    return 0;
  }

  match recipes.get(&ingredient) {
    None => amount,
    Some((subrecipe, yield_amount)) => {
      let times = (amount as f64 / *yield_amount as f64).ceil() as u64;
      let result = subrecipe.iter()
        .map(|(&ingredient, &amount)| {
          get_base_ingredient_amount(recipes, ingredient, amount * times, residues)
        })
        .sum();

      let new_residue = residues.get(&ingredient).cloned().unwrap_or(0) + *yield_amount * times - amount;
      residues.insert(ingredient, new_residue);

      result
    }
  }
}

fn main() {
  let input = get_input().unwrap();
  let recipes = parse_input(&input);

  let one_fuel_ore = get_base_ingredient_amount(&recipes, "FUEL", 1, &mut Recipe::new());
  println!("Part 1: {}", one_fuel_ore);

  let max_ore = 1000000000000u64;
  let mut max_fuel_approx = max_ore / one_fuel_ore;
  let mut cooldown = false;

  loop {
    let ore = get_base_ingredient_amount(&recipes, "FUEL", max_fuel_approx, &mut Recipe::new());

    if ore < max_ore {
      if cooldown {
        break;
      }

      max_fuel_approx += (max_ore - ore) / one_fuel_ore + 1;
    } else {
      cooldown = true;
      max_fuel_approx -= 1;
    }
  }

  println!("Part 2: {}", max_fuel_approx);
}
