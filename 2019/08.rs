use std::fs::File;
use std::io::prelude::*;
use std::collections::HashMap;

type Layer = HashMap<(usize, usize), u8>;

struct LayerIter<T: Iterator<Item = u8>> {
  pixels: T,
  width: usize,
  height: usize
}

impl<T: Iterator<Item = u8>> Iterator for LayerIter<T> {
  type Item = Layer;

  fn next(&mut self) -> Option<Self::Item> {
    let mut layer = Layer::new();

    for y in 0..self.height {
      for x in 0..self.width {
        if let Some(pixel) = self.pixels.next() {
          layer.insert((x, y), pixel);
        } else {
          return None;
        }
      }
    }

    Some(layer)
  }
}

fn get_input() -> std::io::Result<String> {
  let mut file = File::open("08.txt")?;
  let mut contents = String::new();
  file.read_to_string(&mut contents)?;

  Ok(contents)
}

fn parse_layers(input: &str, width: usize, height: usize) -> LayerIter<impl Iterator<Item = u8> + '_> {
  let pixels = input.chars()
    .filter(|c| c.is_ascii_digit())
    .filter_map(|c| c.to_digit(10).map(|d| d as u8));

  LayerIter {
    pixels,
    width,
    height
  }
}

fn merge_layers<T: Iterator<Item = Layer>>(layers: T) -> Layer {
  layers
  .fold(Layer::new(), |mut merged_layer, layer| {
    for (&position, &pixel) in layer.iter() {
      if !merged_layer.contains_key(&position) && pixel != 2 {
        merged_layer.insert(position, pixel);
      }
    }

    merged_layer
  })
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

  let (_, ones, twos) = parse_layers(&input, width, height)
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

  let merged_layer = merge_layers(parse_layers(&input, width, height));

  println!("Part 2:\n{}", render(merged_layer, width, height));
}
