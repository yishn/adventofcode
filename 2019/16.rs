use std::fs::File;
use std::io::prelude::*;

fn get_input() -> std::io::Result<String> {
  let mut file = File::open("16.txt")?;
  let mut contents = String::new();
  file.read_to_string(&mut contents)?;

  Ok(contents)
}

fn parse_signal(input: &str) -> Vec<u8> {
  input.chars()
  .filter(|c| c.is_ascii_digit())
  .filter_map(|c| c.to_digit(10).map(|x| x as u8))
  .collect()
}

fn parse_real_signal(input: &str) -> Vec<u8> {
  let numbers = parse_signal(input);
  let length = numbers.len();

  numbers.into_iter()
  .cycle()
  .take(length * 10000)
  .collect()
}

fn do_phase(signal: &[u8], skipped: usize) -> Vec<u8> {
  let optimization = signal.len() <= skipped;
  let mut prev_number = None;

  (skipped..signal.len() + skipped)
  .map(|i| {
    (0..i - skipped)
    .map(|_| 0)
    .chain(
      [1, 0, -1, 0].into_iter()
      .cloned()
      .flat_map(move |x| (0..=i).map(move |_| x))
      .cycle()
    )
  })
  .enumerate()
  .map(|(i, pattern)| {
    if !optimization || prev_number.is_none() {
      prev_number = Some(
        signal.iter()
        .zip(pattern)
        .map(|(x, y)| *x as i32 * y)
        .sum::<i32>()
      );
    } else {
      prev_number = Some(prev_number.unwrap() - signal[i - 1] as i32);
    }

    prev_number.unwrap()
  })
  .map(|x| (x.abs() % 10) as u8)
  .collect()
}

fn do_phases(n: u32, signal: &[u8], skip: usize) -> Vec<u8> {
  let mut numbers = signal.to_vec();

  for _ in 0..n {
    numbers = do_phase(&numbers, skip);
  }

  numbers
}

fn render_digit_list(numbers: &[u8], skip: usize, take: usize) -> String {
  numbers.iter()
  .cloned()
  .skip(skip)
  .take(take)
  .map(|d| d.to_string())
  .fold(String::new(), |acc, d| acc + &d)
}

fn main() {
  let input = get_input().unwrap();
  let signal = parse_signal(&input);
  let final_output = do_phases(100, &signal, 0);

  println!("Part 1: {}", render_digit_list(&final_output, 0, 8));

  let real_signal = parse_real_signal(&input);
  let skip = render_digit_list(&real_signal, 0, 7).parse::<usize>().unwrap();
  let skipped_signal = real_signal.into_iter().skip(skip).collect::<Vec<_>>();
  let final_output = do_phases(100, &skipped_signal, skip);

  println!("Part 2: {}", render_digit_list(&final_output, 0, 8));
}
