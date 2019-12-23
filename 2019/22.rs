use std::fs::File;
use std::io::prelude::*;

#[derive(Debug, Copy, Clone)]
enum ShuffleOperation {
  DealIntoNewStack,
  CutNCards(isize),
  DealWithIncrement(usize)
}

fn get_input() -> std::io::Result<String> {
  let mut file = File::open("22.txt")?;
  let mut contents = String::new();
  file.read_to_string(&mut contents)?;

  Ok(contents)
}

fn parse_instructions(input: &str) -> Vec<ShuffleOperation> {
  input.lines()
  .filter_map(|line| {
    if line == "deal into new stack" {
      Some(ShuffleOperation::DealIntoNewStack)
    } else if &line[.."cut".len()] == "cut" {
      line["cut".len()..].trim().parse::<isize>().ok()
      .map(|n| ShuffleOperation::CutNCards(n))
    } else if &line[.."deal with increment".len()] == "deal with increment" {
      line["deal with increment".len()..].trim().parse::<usize>().ok()
      .map(|n| ShuffleOperation::DealWithIncrement(n))
    } else {
      None
    }
  })
  .collect()
}

fn extended_gcd(a: isize, b: isize) -> (isize, isize, isize) {
  let mut r = (a, b);
  let mut s = (1, 0);
  let mut t = (0, 1);

  while r.1 != 0 {
    let (old_r, new_r) = r;
    let (old_s, new_s) = s;
    let (old_t, new_t) = t;
    let quotient = old_r / new_r;

    s = (new_s, old_s - quotient * new_s);
    r = (new_r, old_r - quotient * new_r);
    t = (new_t, old_t - quotient * new_t);
  }

  (s.0, t.0, r.0)
}

fn mod_multiply(a: usize, b: usize, m: usize) -> usize {
  ((a as u128 * b as u128) % m as u128) as usize
}

fn parameters_multiply((a, b): (usize, usize), (c, d): (usize, usize), m: usize) -> (usize, usize) {
  (mod_multiply(a, c, m), (mod_multiply(a, d, m) + b) % m)
}

fn parameters_pow(parameters: (usize, usize), p: usize, m: usize) -> (usize, usize) {
  if p == 0 {
    (1, 0)
  } else if p == 1 {
    parameters
  } else if p % 2 == 1 {
    parameters_multiply(parameters, parameters_pow(parameters, p - 1, m), m)
  } else {
    let x = parameters_pow(parameters, p / 2, m);
    parameters_multiply(x, x, m)
  }
}

fn calculate_shuffle_parameters<I>(count: usize, instructions: I) -> (usize, usize)
where I: Iterator<Item = ShuffleOperation> {
  instructions
  .map(|instruction| match instruction {
    ShuffleOperation::DealIntoNewStack => (count - 1, count - 1),
    ShuffleOperation::CutNCards(n) => (1, (count as isize - n) as usize % count),
    ShuffleOperation::DealWithIncrement(n) => (n % count, 0)
  })
  .fold((1, 0), |acc, parameters| {
    parameters_multiply(parameters, acc, count)
  })
}

fn track_card_position((a, b): (usize, usize), card: usize, count: usize) -> usize {
  (mod_multiply(a, card, count) + b) % count
}

fn invert_instructions(count: usize, instructions: &[ShuffleOperation]) -> Vec<ShuffleOperation> {
  let mut inverse_instructions = instructions.to_vec();

  inverse_instructions.reverse();
  inverse_instructions.into_iter()
  .map(|instruction| match instruction {
    ShuffleOperation::DealIntoNewStack => ShuffleOperation::DealIntoNewStack,
    ShuffleOperation::CutNCards(n) => ShuffleOperation::CutNCards(-n),
    ShuffleOperation::DealWithIncrement(n) => {
      let (_, inverse_n, _) = extended_gcd(count as isize, n as isize);
      let inverse_n = if inverse_n < 0 { count - (-inverse_n as usize) } else { inverse_n as usize };
      ShuffleOperation::DealWithIncrement(inverse_n)
    }
  })
  .collect()
}

fn main() {
  let input = get_input().unwrap();

  let instructions = parse_instructions(&input);
  let count = 10007;
  let shuffle_parameters = calculate_shuffle_parameters(count, instructions.iter().cloned());
  let position = track_card_position(shuffle_parameters, 2019, count);

  println!("Part 1: {}", position);

  let times = 101741582076661;
  let count = 119315717514047;
  let inverse_instructions = invert_instructions(count, &instructions);
  let shuffle_parameters = calculate_shuffle_parameters(count, inverse_instructions.iter().cloned());
  let shuffle_parameters = parameters_pow(shuffle_parameters, times, count);
  let card = track_card_position(shuffle_parameters, 2020, count);

  println!("Part 2: {}", card);
}
