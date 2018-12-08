use std::fs::File;
use std::io::prelude::*;
use std::collections::HashMap;

fn get_input() -> std::io::Result<String> {
    let mut file = File::open("08.txt")?;
    let mut contents = String::new();
    file.read_to_string(&mut contents)?;

    Ok(contents)
}

#[derive(Debug)]
struct Node(Vec<usize>);

struct Graph {
    nodes: Vec<Node>,
    children: HashMap<usize, Vec<usize>>
}

impl Graph {
    fn new() -> Graph {
        Graph {
            nodes: vec![],
            children: HashMap::new()
        }
    }
}

fn parse_node(graph: &mut Graph, tokens: &[usize]) -> Option<(usize, usize)> {
    if tokens.len() < 2 {
        return None;
    }

    let children_count = *tokens.get(0).unwrap();
    let metadata_count = *tokens.get(1).unwrap();
    let mut start = 2;
    let mut children_ids = vec![];

    for _ in 0..children_count {
        let (node_id, length) = match parse_node(graph, &tokens[start..]) {
            Some(x) => x,
            _ => return None
        };

        children_ids.push(node_id);
        start += length;
    }

    let length = start + metadata_count;
    let metadata = tokens[start..length].to_vec();

    if metadata.len() != metadata_count {
        return None;
    }

    let node = Node(metadata);
    let node_id = graph.nodes.len();

    graph.nodes.push(node);
    graph.children.insert(node_id, children_ids);

    Some((node_id, length))
}

fn parse_input(input: &str) -> Option<(Graph, usize)> {
    let mut graph = Graph::new();
    let tokens: Vec<usize> = input.split(' ').filter_map(|x| x.parse::<usize>().ok()).collect();

    parse_node(&mut graph, &tokens[..])
    .map(|(root_id, _)| (graph, root_id))
}

fn get_value(graph: &Graph, id: usize) -> usize {
    let metadata = match graph.nodes.get(id) {
        Some(node) => &node.0,
        _ => return 0
    };

    let children = graph.children.get(&id);

    if children.map(|v| v.len()).unwrap_or(0) == 0 {
        metadata.iter().cloned().sum::<usize>()
    } else {
        metadata.iter()
        .map(|&index| {
            children.unwrap()
            .get(index - 1)
            .map(|&child_id| get_value(graph, child_id))
            .unwrap_or(0)
        })
        .sum::<usize>()
    }
}

fn main() {
    let input = get_input().unwrap();
    let (graph, root_id) = match parse_input(input.trim()) {
        Some(x) => x,
        _ => return
    };

    let metadata_sum = graph.nodes.iter()
        .flat_map(|&Node(ref v)| v.iter())
        .cloned()
        .sum::<usize>();

    println!("Part 1: {}", metadata_sum);

    let value = get_value(&graph, root_id);

    println!("Part 2: {}", value);
}
