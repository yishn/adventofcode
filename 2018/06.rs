use std::fs::File;
use std::io::prelude::*;
use std::collections::HashMap;

fn get_input() -> std::io::Result<String> {
    let mut file = File::open("06.txt")?;
    let mut contents = String::new();
    file.read_to_string(&mut contents)?;

    Ok(contents)
}

#[derive(Debug, Copy, Clone, Hash, Eq, PartialEq)]
struct Coord(isize, isize);

fn manhattan_dist(Coord(x1, y1): Coord, Coord(x2, y2): Coord) -> isize {
    (x2 - x1).abs() + (y2 - y1).abs()
}

fn main() {
    let input = get_input().unwrap();
    let pivots: Vec<Coord> = input.lines()
        .filter_map(|line| {
            let mut tokens = line.split(", ").filter_map(|x| x.parse::<isize>().ok());
            tokens.next().and_then(|x| tokens.next().map(|y| Coord(x, y)))
        })
        .collect();

    let min_x = pivots.iter().map(|c| c.0).min().unwrap_or(0);
    let max_x = pivots.iter().map(|c| c.0).max().unwrap_or(0);
    let min_y = pivots.iter().map(|c| c.1).min().unwrap_or(0);
    let max_y = pivots.iter().map(|c| c.1).max().unwrap_or(0);
    let is_inner_area = |Coord(x, y)| x > min_x && x < max_x && y > min_y && y < max_y;

    let all_coords: Vec<Coord> = (min_x..max_x + 1)
        .flat_map(|x| (min_y..max_y + 1).map(move |y| Coord(x, y)))
        .collect();

    let inner_areas = all_coords.iter()
        .cloned()
        .fold(HashMap::new(), |mut acc: HashMap<Coord, usize>, coord| {
            let distances: Vec<isize> = pivots.iter().map(|&c| manhattan_dist(c, coord)).collect();
            let min_dist = distances.iter().cloned().min().unwrap_or(0);
            let mut nearest_pivots = pivots.iter()
                .enumerate()
                .filter(|&(i, _)| distances[i] == min_dist)
                .map(|(_, &c)| c);

            match (nearest_pivots.next(), nearest_pivots.next()) {
                (Some(pivot), None) if is_inner_area(pivot) => {
                    let value = acc.get(&pivot).cloned().unwrap_or(0);
                    acc.insert(pivot, value + 1);
                },
                _ => {}
            }

            acc
        });

    let largest_area = inner_areas.values().cloned().max().unwrap_or(0);

    println!("Part 1: {}", largest_area);

    let safe_area = all_coords.iter()
        .filter(|&&coord| pivots.iter().map(|&c| manhattan_dist(c, coord)).sum::<isize>() < 10000)
        .count();

    println!("Part 2: {}", safe_area);
}
