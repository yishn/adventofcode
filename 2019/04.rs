use std::fs::File;
use std::io::prelude::*;

fn get_input() -> std::io::Result<String> {
  let mut file = File::open("04.txt")?;
  let mut contents = String::new();
  file.read_to_string(&mut contents)?;

  Ok(contents)
}

fn parse_input(input: &str) -> Option<(u32, u32)> {
  let vec = input.split('-')
    .filter_map(|x| x.trim().parse::<u32>().ok())
    .collect::<Vec<_>>();

  if vec.len() != 2 {
    None
  } else {
    Some((vec[0], vec[1]))
  }
}

fn get_digits(n: u32) -> Vec<u8> {
  let mut result = Vec::new();
  let mut n = n;

  while n > 0 {
    let last_digit = (n - n / 10 * 10) as u8;
    n /= 10;

    result.push(last_digit);
  }

  result.reverse();
  result
}

fn is_monotone<T: PartialOrd>(vec: &Vec<T>) -> bool {
  vec.iter().enumerate()
  .all(|(i, d)| i == 0 || &vec[i - 1] <= d)
}

fn is_valid(password: u32) -> bool {
  let digits = get_digits(password);

  digits.iter().enumerate()
    .any(|(i, &d)| i > 0 && d == digits[i - 1])
  && is_monotone(&digits)
}

fn is_actually_valid(password: u32) -> bool {
  let digits = get_digits(password);

  if !is_monotone(&digits) {
    return false;
  }

  let mut change_indices = digits.iter().enumerate()
    .filter(|&(i, &d)| i == 0 || d != digits[i - 1])
    .map(|(i, _)| i)
    .collect::<Vec<_>>();

  change_indices.push(digits.len());
  change_indices.iter().enumerate()
  .any(|(j, &i)| i > 0 && i - change_indices[j - 1] == 2)
}

fn main() {
  let input = get_input().unwrap();
  let (min, max) = parse_input(&input).unwrap();

  let valid_count = (min..=max).filter(|&pw| is_valid(pw)).count();
  println!("Part 1: {}", valid_count);

  let valid_count = (min..=max).filter(|&pw| is_actually_valid(pw)).count();
  println!("Part 2: {}", valid_count);
}
