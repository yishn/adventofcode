use std::fs::File;
use std::io::prelude::*;
use std::collections::{HashSet, HashMap};

fn get_input() -> std::io::Result<String> {
    let mut file = File::open("17.txt")?;
    let mut contents = String::new();
    file.read_to_string(&mut contents)?;

    Ok(contents)
}

type Point = (usize, usize);
type DepthMap = HashSet<Point>;

fn parse(input: &str) -> DepthMap {
    input.lines()
    .flat_map(|line| {
        let parts = line.split(", ")
            .filter_map(|part| {
                let mut tokens = part.split('=');

                match (tokens.next(), tokens.next()) {
                    (Some(coord), Some(value)) => Some((coord, value)),
                    _ => None
                }
            })
            .map(|(coord, value)| (
                coord,
                match value.parse::<usize>() {
                    Ok(x) => vec![x, x],
                    _ => value.split("..").filter_map(|x| x.parse::<usize>().ok()).collect()
                }
            ))
            .filter(|(_, values)| values.len() == 2)
            .fold(HashMap::new(), |mut acc, (coord, values)| {
                acc.insert(coord, values);
                acc
            });

        let (xstart, xend) = parts.get(&"x")
            .map(|v| (v[0], v[1]))
            .unwrap_or((0, 0));

        let (ystart, yend) = parts.get(&"y")
            .map(|v| (v[0], v[1]))
            .unwrap_or((0, 0));

        (xstart..xend + 1)
        .flat_map(move |x| (ystart..yend + 1).map(move |y| (x, y)))
    })
    .collect()
}

fn main() {
    let input = get_input().unwrap();
    let depth_map = parse(&input);

    println!("{:?}", depth_map);
}
