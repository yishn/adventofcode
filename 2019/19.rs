use std::fs::File;
use std::io::prelude::*;
use std::collections::HashMap;

mod intcode;
use intcode::*;

struct TractorBeamLevelIter<'a> {
  program: &'a [i64],
  previous_level: Vec<(u32, u32)>
}

impl<'a> TractorBeamLevelIter<'a> {
  fn new(program: &'a [i64], first_level: Vec<(u32, u32)>) -> TractorBeamLevelIter<'a> {
    TractorBeamLevelIter {
      program,
      previous_level: first_level
    }
  }
}

impl<'a> Iterator for TractorBeamLevelIter<'a> {
  type Item = Vec<(u32, u32)>;

  fn next(&mut self) -> Option<Self::Item> {
    let mut next_level = self.previous_level.iter()
      .cloned()
      .flat_map(|(x, y)| vec![(x + 1, y), (x, y + 1)])
      .filter(|&pos| is_pulling(self.program, pos))
      .collect::<Vec<_>>();

    next_level.dedup();

    if next_level.len() == 0 && self.previous_level.len() > 0 {
      let sum = self.previous_level.first().map(|(x, y)| x + y + 1).unwrap();

      next_level = (0..=sum).map(|x| (x, sum - x)).collect::<Vec<_>>();
    }

    self.previous_level = next_level.clone();
    Some(next_level)
  }
}

fn get_input() -> std::io::Result<String> {
  let mut file = File::open("19.txt")?;
  let mut contents = String::new();
  file.read_to_string(&mut contents)?;

  Ok(contents)
}

fn is_pulling(program: &[i64], (x, y): (u32, u32)) -> bool {
  let mut state = ProgramState::new(program.to_vec());
  let (result, _) = run_program_with_inputs(&mut state, vec![x, y].into_iter().map(|x| x as i64));

  match result.last() {
    Some(&x) => x == 1,
    _ => panic!()
  }
}

fn is_pulling_with_cache(program: &[i64], position: (u32, u32), cache: &mut HashMap<(u32, u32), bool>) -> bool {
  if let Some(&result) = cache.get(&position) {
    return result;
  }

  let result = is_pulling(program, position);
  cache.insert(position, result);

  result
}

fn main() {
  let input = get_input().unwrap();
  let program = input.split(',')
    .filter_map(|x| x.trim().parse::<i64>().ok())
    .collect::<Vec<_>>();

  let mut pull_cache = HashMap::new();
  let pull_count = (0..50).flat_map(|x| (0..50).map(move |y| (x, y)))
    .filter(|&pos| is_pulling_with_cache(&program, pos, &mut pull_cache))
    .count();

  println!("Part 1: {}", pull_count);

  let beam_level_iter = TractorBeamLevelIter::new(&program, vec![(0, 0)]);

  let square_fit_position = beam_level_iter
    .enumerate()
    .skip_while(|(_, level)| level.len() < 95)
    .flat_map(|(_, level)| level)
    .find(|&(x, y)| {
      is_pulling_with_cache(&program, (x + 99, y), &mut pull_cache)
      && is_pulling_with_cache(&program, (x, y + 99), &mut pull_cache)
    });

  square_fit_position.map(|(x, y)| {
    println!("Part 2: {}", x * 10000 + y);
  });
}
