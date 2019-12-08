use std::fs::File;
use std::io::prelude::*;
use std::collections::HashMap;

type Layer = HashMap<(usize, usize), u8>;

fn get_input() -> std::io::Result<String> {
  let mut file = File::open("08.txt")?;
  let mut contents = String::new();
  file.read_to_string(&mut contents)?;

  Ok(contents)
}

fn parse_layers(input: &str, width: usize, height: usize) -> Vec<Layer> {
  let mut layers = vec![];
  let mut digits = input.chars()
    .filter(|c| c.is_ascii_digit())
    .filter_map(|c| c.to_digit(10).map(|d| d as u8));

  loop {
    let mut layer = Layer::new();

    for y in 0..height {
      for x in 0..width {
        if let Some(d) = digits.next() {
          layer.insert((x, y), d);
        } else {
          return layers;
        }
      }
    }

    layers.push(layer);
  }
}

fn merge_layers(layers: &Vec<Layer>, width: usize, height: usize) -> Layer {
  let mut result = Layer::new();

  for y in 0..height {
    for x in 0..width {
      result.insert(
        (x, y),
        layers.iter()
        .filter_map(|layer| layer.get(&(x, y)).cloned())
        .filter(|&pixel| pixel != 2)
        .next()
        .unwrap_or(2)
      );
    }
  }

  result
}

fn render(layer: Layer, width: usize, height: usize) -> String {
  let mut result = String::new();

  for y in 0..height {
    for x in 0..width {
      let output = match layer.get(&(x, y)) {
        Some(&0) => ' ',
        _ => '#'
      };

      result.push(output);
    }

    result.push('\n');
  }

  result
}

fn main() {
  let input = get_input().unwrap();
  let width = 25;
  let height = 6;
  let layers = parse_layers(&input, width, height);

  let (_, ones, twos) = layers.iter()
    .map(|layer| {
      layer.iter()
      .filter_map(|(_, &pixel)| match pixel {
        0 => Some((1, 0, 0)),
        1 => Some((0, 1, 0)),
        2 => Some((0, 0, 1)),
        _ => None
      })
      .fold((0, 0, 0), |mut acc, (a, b, c)| {
        acc.0 += a;
        acc.1 += b;
        acc.2 += c;
        acc
      })
    })
    .min_by_key(|&(zeros, _, _)| zeros)
    .unwrap();

  println!("Part 1: {}", ones * twos);

  let merged_layer = merge_layers(&layers, width, height);

  println!("Part 2:\n{}", render(merged_layer, width, height));
}
