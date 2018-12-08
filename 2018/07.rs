use std::fmt;
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

struct Timetable<'a, T: 'a>
where T: Hash + Eq {
    durations: &'a HashMap<T, usize>,
    by_time: HashMap<usize, HashMap<usize, T>>,
    stop_times: HashMap<T, usize>
}

impl<'a, T: 'a> Timetable<'a, T>
where T: Hash + Eq + Copy {
    fn new(durations: &HashMap<T, usize>) -> Timetable<T> {
        Timetable {
            durations,
            by_time: HashMap::new(),
            stop_times: HashMap::new()
        }
    }

    fn allocate(&mut self, job: T, worker: usize, time: usize) {
        let duration = self.durations.get(&job).cloned().unwrap_or(1);

        if self.stop_times.contains_key(&job) {
            return;
        }

        for t in time..time + duration {
            if !self.by_time.contains_key(&t) {
                self.by_time.insert(t, HashMap::new());
            }

            let worker_data = self.by_time.get_mut(&t).unwrap();
            worker_data.insert(worker, job);
        }

        self.stop_times.insert(job, time + duration);
    }
}

impl<'a> fmt::Debug for Timetable<'a, char> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        writeln!(f, "Timetable {{")?;

        for t in 0..self.by_time.keys().cloned().max().unwrap_or(0) + 1 {
            write!(f, "  ({:4}) ", t)?;

            if let Some(worker_data) = self.by_time.get(&t) {
                for id in 0..worker_data.keys().cloned().max().unwrap_or(0) + 1 {
                    write!(f, "{:?} ", worker_data.get(&id).cloned().unwrap_or(' '))?;
                }
            }

            writeln!(f, "")?;
        }

        write!(f, "}}")?;
        Ok(())
    }
}

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

fn get_root_nodes<T>(graph: &Graph<T>) -> Vec<T>
where T: Eq + Hash + Copy {
    graph.keys()
    .cloned()
    .filter(|&k| {
        graph.values()
        .flat_map(|children| children.iter())
        .all(|&child| child != k)
    })
    .collect()
}

fn get_order<T>(mut graph: Graph<T>) -> Vec<T>
where T: Eq + Hash + Copy + Ord {
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

fn create_timetable<T>(mut graph: Graph<T>, durations: &HashMap<T, usize>, workers: usize) -> Timetable<T>
where T: Eq + Hash + Copy + Ord {
    let mut result = Timetable::new(durations);
    let mut root_nodes;
    let mut busy = vec![];
    let mut t = 0;

    loop {
        if !result.by_time.contains_key(&t) {
            result.by_time.insert(t, HashMap::new());
        }

        // Remove jobs that have stopped

        let stopped: Vec<T> = busy.iter().cloned()
            .filter(|&job| result.stop_times.get(&job).cloned().unwrap() <= t)
            .collect();

        for node in stopped.iter() {
            graph.remove(node);
        }

        busy.retain(|job| !stopped.contains(job));

        // Get available jobs

        root_nodes = get_root_nodes(&graph);
        root_nodes.retain(|job| !busy.contains(job));
        root_nodes.sort();

        if root_nodes.len() == 0 {
            if busy.len() == 0 {
                break;
            } else {
                t += 1;
                continue;
            }
        }

        // Allocate jobs

        let free_workers: Vec<usize> = {
            let worker_data = result.by_time.get(&t).unwrap();

            (0..workers)
            .filter(|id| !worker_data.contains_key(id))
            .collect()
        };

        for (job, worker) in root_nodes.iter().cloned().zip(free_workers.iter().cloned()) {
            result.allocate(job, worker, t);
            busy.push(job);
        }
    }

    result
}

fn main() {
    let input = get_input().unwrap();
    let graph = parse_input(&input);
    let order = get_order(graph.clone());

    println!("Part 1: {}", order.into_iter().fold(String::new(), |mut acc, x| {
        acc.push(x);
        acc
    }));

    let alphabet = "ABCDEFGHIJKLMNOPQRSTUVWXYZ";
    let durations = alphabet.chars()
        .enumerate()
        .fold(HashMap::new(), |mut acc, (i, c)| {
            acc.insert(c, i + 61);
            acc
        });
    let timetable = create_timetable(graph, &durations, 5);

    println!("Part 2: {}", timetable.by_time.keys().cloned().max().unwrap_or(0));
}
