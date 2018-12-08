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

fn parse_node<T>(graph: &mut Graph, tokens: &mut T) -> Option<usize>
where T: Iterator<Item = usize> {
    let (children_count, metadata_count) = match (tokens.next(), tokens.next()) {
        (Some(x), Some(y)) => (x, y),
        _ => return None
    };

    let children_ids: Vec<usize> = (0..children_count)
        .filter_map(|_| parse_node(graph, tokens))
        .collect();

    let metadata: Vec<usize> = (0..metadata_count)
        .filter_map(|_| tokens.next())
        .collect();

    let node = Node(metadata);
    let node_id = graph.nodes.len();

    graph.nodes.push(node);
    graph.children.insert(node_id, children_ids);

    Some(node_id)
}

fn parse_input(input: &str) -> Option<(Graph, usize)> {
    let mut graph = Graph::new();
    let mut tokens = input.split(' ').filter_map(|x| x.parse::<usize>().ok());

    parse_node(&mut graph, &mut tokens)
    .map(|root_id| (graph, root_id))
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
