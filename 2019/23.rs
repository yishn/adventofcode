use std::fs::File;
use std::io::prelude::*;
use std::collections::VecDeque;

mod intcode;
use intcode::*;

#[derive(Debug, Copy, Clone)]
enum NetworkAction {
  Continue,
  Halt
}

#[derive(Debug, Clone)]
struct NetworkPackage {
  from: usize,
  to: usize,
  x: i64,
  y: i64
}

fn get_input() -> std::io::Result<String> {
  let mut file = File::open("23.txt")?;
  let mut contents = String::new();
  file.read_to_string(&mut contents)?;

  Ok(contents)
}

fn simulate_network<F>(program: &[i64], count: usize, on_package: F)
where F: Fn(&NetworkPackage) -> NetworkAction {
  let mut nics = (0..count)
    .map(|_| ProgramState::new(program.to_vec()))
    .collect::<Vec<_>>();

  let mut started = nics.iter()
    .map(|_| false)
    .collect::<Vec<_>>();

  let mut queues = nics.iter()
    .map(|_| VecDeque::<NetworkPackage>::new())
    .collect::<Vec<_>>();

  loop {
    for id in 0..count {
      let inputs = {
        let mut inputs = if !started[id] {
          started[id] = true;
          vec![id as i64]
        } else {
          vec![]
        };

        inputs.append(
          &mut queues[id].pop_front()
          .map(|package| vec![package.x, package.y])
          .unwrap_or_else(|| vec![-1])
        );

        inputs
      };

      let (outputs, _) = run_program_with_inputs(&mut nics[id], inputs.into_iter());
      let mut outputs = outputs.into_iter();

      while let (Some(to), Some(x), Some(y)) = (outputs.next(), outputs.next(), outputs.next()) {
        let package = NetworkPackage {
          from: id,
          to: to as usize,
          x,
          y
        };

        if let NetworkAction::Halt = on_package(&package) {
          return;
        }

        queues[to as usize].push_back(package);
      }
    }
  }
}

fn main() {
  let input = get_input().unwrap();
  let program = input.split(',')
    .filter_map(|x| x.trim().parse::<i64>().ok())
    .collect::<Vec<_>>();

  simulate_network(&program, 50, |package| {
    if package.to == 255 {
      println!("Part 1: {}", package.y);
      NetworkAction::Halt
    } else {
      NetworkAction::Continue
    }
  });
}
