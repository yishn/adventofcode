use std::fs::File;
use std::io::prelude::*;
use std::collections::{HashMap, HashSet};
use std::hash::Hash;

fn get_input() -> std::io::Result<String> {
    let mut file = File::open("07.txt")?;
    let mut contents = String::new();
    file.read_to_string(&mut contents)?;

    Ok(contents)
}

type Graph<T> = HashMap<T, HashSet<T>>;
type Timetable<T> = HashMap<usize, HashMap<usize, T>>;

fn parse_input(input: &str) -> Graph<char> {
    fn first_char(x: &str) -> Option<char> {
        x.chars().next()
    }

    input.lines()
    .map(|line| line.split(' '))
    .filter_map(|mut tokens| {
        tokens.nth(1).and_then(first_char)
        .and_then(|x| tokens.nth(5).and_then(first_char).map(|y| (x, y)))
    })
    .fold(Graph::new(), |mut graph, (x, y)| {
        if !graph.contains_key(&x) {
            graph.insert(x, HashSet::new());
        }

        let mut children = graph.remove(&x).unwrap();
        children.insert(y);

        for &child in children.iter() {
            if !graph.contains_key(&child) {
                graph.insert(child, HashSet::new());
            }
        }

        graph.insert(x, children);
        graph
    })
}

fn get_root_nodes<T: Eq + Hash + Copy>(graph: &Graph<T>) -> Vec<T> {
    graph.keys()
    .cloned()
    .filter(|&k| {
        graph.values()
        .flat_map(|children| children.iter())
        .all(|&child| child != k)
    })
    .collect()
}

fn get_order<T: Eq + Hash + Copy + Ord>(mut graph: Graph<T>) -> Vec<T> {
    let mut result = vec![];
    let mut root_nodes;

    loop {
        root_nodes = get_root_nodes(&graph);
        root_nodes.sort();

        if root_nodes.len() == 0 {
            break;
        }

        let node = root_nodes.remove(0);

        graph.remove(&node);
        result.push(node);
    }

    result
}

fn allocate<T>(timetable: &mut Timetable<T>, node: T, worker: usize, time: usize, duration: usize)
where T: Copy {
    for t in time..time + duration {
        if !timetable.contains_key(&t) {
            timetable.insert(t, HashMap::new());
        }

        let workers = timetable.get_mut(&t).unwrap();
        workers.insert(worker, node);
    }
}

fn has_time<T>(timetable: &mut Timetable<T>, worker: usize, time: usize) -> bool {
    timetable.get(&time)
    .and_then(|workers| workers.get(&worker))
    .is_some()
}

fn create_timetable<T>(graph: Graph<T>, times: &HashMap<T, usize>, workers: usize) -> Timetable<T>
where T: Eq + Hash + Copy + Ord {
    let mut result = Timetable::new();
    let mut root_nodes;

    loop {
        root_nodes = get_root_nodes(&graph);
        root_nodes.sort();
    }

    result
}

fn main() {
    let input = get_input().unwrap();
    let graph = parse_input(&input);
    let order = get_order(graph);

    println!("Part 1: {}", order.into_iter().fold(String::new(), |mut acc, x| {
        acc.push(x);
        acc
    }));
}
