use std::fs::File;
use std::io::prelude::*;
use std::collections::{HashMap, hash_map::DefaultHasher};
use std::hash::{Hasher, Hash};

fn get_input() -> std::io::Result<String> {
    let mut file = File::open("18.txt")?;
    let mut contents = String::new();
    file.read_to_string(&mut contents)?;

    Ok(contents)
}

#[derive(Debug, Copy, Clone, Eq, PartialEq, Hash)]
enum Terrain {
    Open,
    Trees,
    Lumberyard
}

type Point = (isize, isize);
type Landscape = HashMap<Point, Terrain>;

fn parse(input: &str) -> Landscape {
    input.lines()
    .enumerate()
    .flat_map(|(y, line)| line.chars().enumerate().map(move |(x, c)| ((x as isize, y as isize), c)))
    .map(|(k, c)| (k, match c {
        '|' => Terrain::Trees,
        '#' => Terrain::Lumberyard,
        _ => Terrain::Open
    }))
    .fold(Landscape::new(), |mut acc, (k, v)| {
        acc.insert(k, v);
        acc
    })
}

fn get_neighbors((x, y): Point, landscape: &Landscape) -> Vec<Point> {
    (x - 1..x + 2)
    .flat_map(|x| (y - 1..y + 2).map(move |y| (x, y)))
    .filter(|&p| p != (x, y) && landscape.contains_key(&p))
    .collect()
}

fn evolve(landscape: &Landscape) -> Landscape {
    landscape.iter()
    .map(|(&p, &terrain)| {
        let neighbors = get_neighbors(p, &landscape);

        (p, match terrain {
            Terrain::Open => {
                if neighbors.iter().filter(|&n| landscape[n] == Terrain::Trees).count() >= 3 {
                    Terrain::Trees
                } else {
                    Terrain::Open
                }
            },
            Terrain::Trees => {
                if neighbors.iter().filter(|&n| landscape[n] == Terrain::Lumberyard).count() >= 3 {
                    Terrain::Lumberyard
                } else {
                    Terrain::Trees
                }
            },
            Terrain::Lumberyard => {
                if neighbors.iter().any(|n| landscape[n] == Terrain::Lumberyard)
                && neighbors.iter().any(|n| landscape[n] == Terrain::Trees) {
                    Terrain::Lumberyard
                } else {
                    Terrain::Open
                }
            }
        })
    })
    .fold(Landscape::new(), |mut acc, (k, v)| {
        acc.insert(k, v);
        acc
    })
}

fn get_resource_value(landscape: &Landscape) -> usize {
    let lumberyards = landscape.values().filter(|&&t| t == Terrain::Lumberyard).count();
    let wooded = landscape.values().filter(|&&t| t == Terrain::Trees).count();

    lumberyards * wooded
}

fn get_hash(landscape: &Landscape, points: &[Point]) -> u64 {
    let mut hasher = DefaultHasher::new();

    for p in points.iter() {
        (*p, landscape[p]).hash(&mut hasher);
    }

    hasher.finish()
}

fn main() {
    let input = get_input().unwrap();
    let original_landscape = parse(&input);
    let landscape = (0..10).fold(original_landscape.clone(), |acc, _| evolve(&acc));

    println!("Part 1: {}", get_resource_value(&landscape));

    let mut landscape = original_landscape.clone();
    let mut equal_indices = (0, 0);
    let mut history = Vec::new();

    let mut points = original_landscape.keys().cloned().collect::<Vec<_>>();
    points.sort();

    for i in 0.. {
        landscape = evolve(&landscape);
        let hash = get_hash(&landscape, &points);

        match history.iter().position(|&x| x == hash) {
            Some(j) => {
                equal_indices = (j, i);
                break;
            },
            _ => history.push(hash)
        }
    }

    let cycle = equal_indices.1 - equal_indices.0;
    let index = (1000000000 - equal_indices.0) % cycle - 1;
    let landscape = (0..index).fold(landscape, |acc, _| evolve(&acc));

    println!("Part 2: {}", get_resource_value(&landscape));
}
