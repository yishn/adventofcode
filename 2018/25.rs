use std::fs::File;
use std::io::prelude::*;
use std::collections::{HashMap, HashSet};

fn get_input() -> std::io::Result<String> {
    let mut file = File::open("25.txt")?;
    let mut contents = String::new();
    file.read_to_string(&mut contents)?;

    Ok(contents)
}

type Coordinate = (isize, isize, isize, isize);
type ConstellationGraph = HashMap<Coordinate, Vec<Coordinate>>;

fn parse(input: &str) -> Vec<Coordinate> {
    input.lines()
    .map(|line| line.split(',').filter_map(|x| x.parse::<isize>().ok()).collect::<Vec<_>>())
    .filter(|v| v.len() == 4)
    .map(|v| (v[0], v[1], v[2], v[3]))
    .collect()
}

fn manhattan_dist((x1, y1, z1, t1): Coordinate, (x2, y2, z2, t2): Coordinate) -> usize {
    ((x2 - x1).abs() + (y2 - y1).abs() + (z2 - z1).abs() + (t2 - t1).abs()) as usize
}

fn get_constellation_graph(coordinates: &[Coordinate]) -> ConstellationGraph {
    coordinates.iter()
    .map(|&coord| (
        coord,
        coordinates.iter()
            .cloned()
            .filter(|&c| coord != c && manhattan_dist(coord, c) <= 3)
            .collect::<Vec<_>>()
    ))
    .fold(ConstellationGraph::new(), |mut acc, (k, v)| {
        acc.insert(k, v);
        acc
    })
}

fn get_connected_components(graph: &ConstellationGraph) -> HashMap<Coordinate, Coordinate> {
    let mut remaining_vertices = graph.keys().cloned().collect::<HashSet<_>>();
    let mut pivots = HashMap::new();

    while remaining_vertices.len() > 0 {
        let pivot = remaining_vertices.iter().next().cloned().unwrap();
        let mut stack = vec![pivot];

        while let Some(v) = stack.pop() {
            if !remaining_vertices.contains(&v) {
                continue;
            }

            pivots.insert(v, pivot);
            remaining_vertices.remove(&v);

            for &neighbor in graph[&v].iter() {
                stack.push(neighbor);
            }
        }
    }

    pivots
}

fn main() {
    let input = get_input().unwrap();
    let coordinates = parse(&input);
    let graph = get_constellation_graph(&coordinates);
    let components = get_connected_components(&graph);
    let components_count = components.values().collect::<HashSet<_>>().len();

    println!("Part 1: {}", components_count);
}
