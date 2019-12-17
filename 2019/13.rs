use std::fs::File;
use std::io::prelude::*;
use std::collections::HashMap;
use std::{fmt, env, thread, time};

mod intcode;
use intcode::{run_program, ProgramResult, ProgramState};

#[derive(Debug, PartialEq, Copy, Clone)]
enum Tile {
  Empty,
  Wall,
  Block,
  HorizontalPaddle,
  Ball
}

impl fmt::Display for Tile {
  fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
    write!(f, "{}", match self {
      Tile::Empty => "  ",
      Tile::Wall => "##",
      Tile::Block => "[]",
      Tile::HorizontalPaddle => "==",
      Tile::Ball => "()",
    })
  }
}

type TileGrid = HashMap<(i64, i64), Tile>;

fn get_input() -> std::io::Result<String> {
  let mut file = File::open("13.txt")?;
  let mut contents = String::new();
  file.read_to_string(&mut contents)?;

  Ok(contents)
}

fn output_screen(state: &mut ProgramState, input: Option<i64>) -> (TileGrid, Option<i64>, bool) {
  let mut result = TileGrid::new();
  let mut score = None;
  let mut input_iter = input.into_iter();
  let mut halted = false;

  loop {
    let mut get = |input| run_program(state, input);

    match (get(input_iter.next()), get(None), get(None)) {
      (ProgramResult::Output(x), ProgramResult::Output(y), ProgramResult::Output(value)) => {
        if (x, y) == (-1, 0) {
          score = Some(value);
        } else {
          result.insert((x, y), match value {
            0 => Tile::Empty,
            1 => Tile::Wall,
            2 => Tile::Block,
            3 => Tile::HorizontalPaddle,
            4 => Tile::Ball,
            _ => panic!()
          });
        }
      },
      (ProgramResult::WaitForInput, _, _) => break,
      (ProgramResult::Halt, _, _) => {
        halted = true;
        break;
      },
      _ => panic!()
    };
  }

  (result, score, halted)
}

fn render_screen(screen: &TileGrid) -> String {
  let mut result = String::new();
  let bounds = screen.keys().cloned()
    .fold((None, None, None, None), |(min_x, max_x, min_y, max_y), (x, y)| {
      (
        [min_x, Some(x)].into_iter().filter_map(|&x| x).min(),
        [max_x, Some(x)].into_iter().filter_map(|&x| x).max(),
        [min_y, Some(y)].into_iter().filter_map(|&y| y).min(),
        [max_y, Some(y)].into_iter().filter_map(|&y| y).max()
      )
    });

  if let (Some(min_x), Some(max_x), Some(min_y), Some(max_y)) = bounds {
    for y in min_y..=max_y {
      for x in min_x..=max_x {
        let tile = screen.get(&(x, y)).cloned().unwrap_or(Tile::Empty);

        result.push_str(&format!("{}", tile));
      }

      result.push('\n');
    }
  }

  result
}

fn play_game(state: &mut ProgramState, print: bool) -> i64 {
  fn get_tile_position(screen: &TileGrid, tile: Tile) -> (i64, i64) {
    screen.iter()
    .find(|&(_, &t)| t == tile)
    .map(|(&position, _)| position)
    .unwrap()
  }

  let (mut screen, mut score, _) = output_screen(state, None);

  loop {
    let paddle_position = get_tile_position(&screen, Tile::HorizontalPaddle);
    let ball_position = get_tile_position(&screen, Tile::Ball);
    let joystick = (ball_position.0 - paddle_position.0).signum();
    let (screen_update, score_update, halted) = output_screen(state, Some(joystick));

    for (&(x, y), &tile) in screen_update.iter() {
      screen.insert((x, y), tile);
    }

    if let Some(_) = score_update {
      score = score_update;
    }

    if print {
      println!("// Score: {}\n{}\n", score.unwrap_or(0), render_screen(&screen));
      thread::sleep(time::Duration::from_millis(50));
    }

    if halted {
      break;
    }
  }

  score.unwrap_or(0)
}

fn main() {
  let input = get_input().unwrap();
  let numbers = input.split(',')
    .filter_map(|x| x.trim().parse::<i64>().ok())
    .collect::<Vec<_>>();

  let (screen, _, _) = output_screen(&mut ProgramState::new(numbers.clone()), None);
  let block_tile_count = screen.iter()
    .filter(|&(_, &tile)| match tile {
      Tile::Block => true,
      _ => false
    })
    .count();

  println!("Part 1: {}", block_tile_count);

  let mut program = numbers.clone();
  program[0] = 2;

  let print_game = env::args().any(|s| s == "--print");
  let score = play_game(&mut ProgramState::new(program), print_game);

  println!("Part 2: {}", score);
}
