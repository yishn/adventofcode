use std::fs::File;
use std::io::prelude::*;
use std::collections::HashMap;

#[derive(Debug, Copy, Clone)]
enum Tile<T> {
  Wall,
  Passage,
  Door(T),
  Key(T)
}

type Position = (usize, usize);
type Labyrinth<T> = HashMap<Position, Tile<T>>;

fn get_input() -> std::io::Result<String> {
  let mut file = File::open("18.txt")?;
  let mut contents = String::new();
  file.read_to_string(&mut contents)?;

  Ok(contents)
}

fn parse_labyrinth(input: &str) -> (Labyrinth<char>, Option<Position>) {
  let labyrinth = input.lines()
    .enumerate()
    .flat_map(|(y, line)| {
      line.chars()
      .enumerate()
      .map(move |(x, c)| (
        (x, y),
        match c {
          '#' => Tile::Wall,
          '@' => Tile::Passage,
          c if c.is_ascii_uppercase() => Tile::Door(c.to_ascii_lowercase()),
          c if c.is_ascii_lowercase() => Tile::Key(c),
          _ => Tile::Passage
        }
      ))
    })
    .collect::<Labyrinth<_>>();

  let entrance = input.lines()
    .enumerate()
    .find_map(|(y, line)| {
      line.chars()
      .position(|c| c == '@')
      .map(|x| (x, y))
    });

  (labyrinth, entrance)
}

fn main() {
  let input = get_input().unwrap();
  let labyrinth = parse_labyrinth(&input);

  println!("{:?}", labyrinth);
}
