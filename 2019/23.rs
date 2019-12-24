use std::fs::File;
use std::io::prelude::*;
use std::collections::VecDeque;

mod intcode;
use intcode::*;

#[derive(Debug, Clone)]
struct NetworkPackage {
  from: usize,
  to: usize,
  x: i64,
  y: i64
}

#[derive(Debug)]
enum NetworkEvent {
  OnPackage(NetworkPackage),
  OnTick
}

struct Network {
  nics: Vec<ProgramState>,
  queues: Vec<VecDeque<NetworkPackage>>,
  started: Vec<bool>,
  idle: Vec<bool>,
  event_queue: VecDeque<NetworkEvent>
}

impl Network {
  fn new(program: &[i64], count: usize) -> Network {
    let nics = (0..count)
      .map(|_| ProgramState::new(program.to_vec()))
      .collect::<Vec<_>>();
    let queues = nics.iter()
      .map(|_| VecDeque::<NetworkPackage>::new())
      .collect::<Vec<_>>();
    let started = nics.iter()
      .map(|_| false)
      .collect::<Vec<_>>();
    let idle = nics.iter()
      .map(|_| true)
      .collect::<Vec<_>>();

    Network {
      nics, queues, started, idle,
      event_queue: VecDeque::new()
    }
  }
}

impl Iterator for Network {
  type Item = NetworkEvent;

  fn next(&mut self) -> Option<NetworkEvent> {
    if let Some(event) = self.event_queue.pop_front() {
      return Some(event);
    }

    let count = self.nics.len();

    for id in 0..count {
      let (inputs, has_incoming_package) = {
        let mut inputs = if !self.started[id] {
          self.started[id] = true;
          vec![id as i64]
        } else {
          vec![]
        };

        let has_incoming_package = self.queues[id].len() > 0;

        inputs.append(
          &mut self.queues[id].pop_front()
          .map(|package| vec![package.x, package.y])
          .unwrap_or_else(|| vec![-1])
        );

        (inputs, has_incoming_package)
      };

      let (outputs, _) = run_program_with_inputs(&mut self.nics[id], inputs.into_iter());
      let mut outputs = outputs.into_iter();
      let mut has_outgoing_package = false;

      while let (Some(to), Some(x), Some(y)) = (outputs.next(), outputs.next(), outputs.next()) {
        has_outgoing_package = true;

        let package = NetworkPackage {
          from: id,
          to: to as usize,
          x, y
        };

        self.event_queue.push_back(NetworkEvent::OnPackage(package.clone()));

        if let Some(queue) = self.queues.get_mut(to as usize) {
          queue.push_back(package);
        }
      }

      self.idle[id] = !has_incoming_package && !has_outgoing_package;
    }

    Some(self.event_queue.pop_front().unwrap_or(NetworkEvent::OnTick))
  }
}

fn get_input() -> std::io::Result<String> {
  let mut file = File::open("23.txt")?;
  let mut contents = String::new();
  file.read_to_string(&mut contents)?;

  Ok(contents)
}

fn main() {
  let input = get_input().unwrap();
  let program = input.split(',')
    .filter_map(|x| x.trim().parse::<i64>().ok())
    .collect::<Vec<_>>();

  let mut print_part1 = true;
  let mut nat = None;
  let mut last_nat = None;
  let mut network = Network::new(&program, 50);

  while let Some(event) = network.next() {
    match event {
      NetworkEvent::OnPackage(package) => {
        if package.to == 255 {
          if print_part1 {
            println!("Part 1: {}", package.y);
            print_part1 = false;
          }

          nat = Some((package.x, package.y));
        }
      },
      NetworkEvent::OnTick => {
        if network.idle.iter().all(|&x| x) {
          if let Some((x, y)) = nat {
            match last_nat {
              Some((_, last_y)) if last_y == y => {
                println!("Part 2: {}", y);
                return;
              },
              _ => {}
            }

            network.queues[0].push_back(NetworkPackage {
              from: 255,
              to: 0,
              x, y
            });

            last_nat = nat;
            nat = None;
          }
        }
      }
    }
  }
}
