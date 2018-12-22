use std::fs::File;
use std::io::prelude::*;
use std::collections::{HashMap, HashSet, BinaryHeap};
use std::cmp::Ordering;

fn get_input() -> std::io::Result<String> {
    let mut file = File::open("22.txt")?;
    let mut contents = String::new();
    file.read_to_string(&mut contents)?;

    Ok(contents)
}

type Point = (isize, isize);
type Map = HashMap<Point, isize>;

#[derive(Debug, Hash, Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
enum Gear {
    Climbing,
    Torch,
    Neither
}

#[derive(Debug, Hash, Copy, Clone, Eq, PartialEq, Ord)]
struct State(usize, Point, Gear);

impl PartialOrd for State {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some((-(self.0 as isize), self.1, self.2).cmp(&(-(other.0 as isize), other.1, other.2)))
    }
}

fn parse(input: &str) -> Option<(isize, Point)> {
    let mut numbers = input.lines()
        .filter_map(|line| line.split(": ").nth(1))
        .map(|line| line.split(',').filter_map(|x| x.parse::<isize>().ok()).collect::<Vec<_>>());

    match (numbers.next(), numbers.next()) {
        (Some(ref x), Some(ref y)) if y.len() >= 2 => {
            Some((x[0], (y[0], y[1])))
        },
        _ => None
    }
}

fn get_erosion_levels(depth: isize, target: Point) -> Map {
    let mut map = Map::new();

    for x in 0..3 * target.0 + 1 {
        for y in 0..3 * target.1 + 1 {
            let index = match (x, y) {
                p if p == target => 0,
                (0, 0) => 0,
                (x, 0) => 16807 % 20183 * x % 20183,
                (0, y) => 48271 % 20183 * y % 20183,
                (x, y) => map.get(&(x - 1, y)).unwrap() % 20183 * map.get(&(x, y - 1)).unwrap() % 20183
            };

            map.insert((x, y), (index + depth) % 20183);
        }
    }

    map
}

fn get_region_map(depth: isize, target: Point) -> Map {
    get_erosion_levels(depth, target).iter()
    .map(|(&k, &v)| (k, v % 3))
    .fold(Map::new(), |mut acc, (k, v)| {
        acc.insert(k, v);
        acc
    })
}

fn get_neighbors(map: &Map, (x, y): Point) -> Vec<Point> {
    [(x - 1, y), (x + 1, y), (x, y - 1), (x, y + 1)].into_iter()
    .cloned()
    .filter(|p| map.contains_key(p))
    .collect()
}

fn get_usable_gear(region_type: isize) -> Vec<Gear> {
    match region_type {
        0 => vec![Gear::Climbing, Gear::Torch],
        1 => vec![Gear::Climbing, Gear::Neither],
        2 => vec![Gear::Torch, Gear::Neither],
        _ => vec![]
    }
}

fn get_fastest_route(map: &Map, target: Point) -> Option<usize> {
    let mut heap = BinaryHeap::new();
    let mut visited = HashSet::new();

    heap.push(State(0, (0, 0), Gear::Torch));

    while let Some(state) = heap.pop() {
        if visited.contains(&(state.1, state.2)) {
            continue;
        } else {
            visited.insert((state.1, state.2));
        }

        if state.1 == target && state.2 == Gear::Torch {
            return Some(state.0);
        }

        let next_states = get_neighbors(map, state.1).into_iter()
            .filter(|pos| get_usable_gear(map[pos]).contains(&state.2))
            .map(|pos| State(state.0 + 1, pos, state.2))
            .chain(
                get_usable_gear(map[&state.1])
                .into_iter()
                .filter(|&gear| gear != state.2)
                .map(|gear| State(state.0 + 7, state.1, gear))
            )
            .collect::<Vec<_>>();

        for next in next_states.into_iter() {
            heap.push(next);
        }
    }

    None
}

fn main() {
    let input = get_input().unwrap();
    let (depth, target) = parse(&input).unwrap();
    let region_map = get_region_map(depth, target);

    println!("Part 1: {}",
        region_map.iter()
        .filter(|(&(x, y), _)| x <= target.0 && y <= target.1)
        .map(|(_, &v)| v)
        .sum::<isize>()
    );

    get_fastest_route(&region_map, target)
    .map(|x| println!("Part 2: {}", x));
}
