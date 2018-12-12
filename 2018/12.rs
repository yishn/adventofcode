use std::fs::File;
use std::io::prelude::*;
use std::collections::{HashSet, HashMap};

fn get_input() -> std::io::Result<String> {
    let mut file = File::open("12.txt")?;
    let mut contents = String::new();
    file.read_to_string(&mut contents)?;

    Ok(contents)
}

type State = HashSet<isize>;
type Instructions = HashMap<[bool; 5], bool>;

fn pattern_from_vec(vec: &Vec<bool>) -> [bool; 5] {
    let mut pattern = [false; 5];

    if vec.len() == 5 {
        pattern.copy_from_slice(&vec[0..5]);
    }

    pattern
}

fn parse(input: &str) -> (State, Instructions) {
    let mut lines = input.lines();

    let state = lines.next()
        .and_then(|first_line| first_line.split(' ').nth(2))
        .unwrap_or("")
        .chars()
        .enumerate()
        .filter_map(|(i, c)| match c {
            '#' => Some(i as isize),
            _ => None
        })
        .collect::<State>();

    let instructions = lines
        .map(|line| line.split(" => "))
        .filter_map(|mut tokens| match (tokens.next(), tokens.next()) {
            (Some(left), Some(right)) => {
                let pattern = pattern_from_vec(
                    &left.chars()
                    .map(|c| c == '#')
                    .collect::<Vec<_>>()
                );

                Some((pattern, right == "#"))
            },
            _ => None
        })
        .fold(Instructions::new(), |mut acc, (pattern, value)| {
            acc.insert(pattern, value);
            acc
        });

    (state, instructions)
}

fn evolve_gen(state: &State, instructions: &Instructions) -> State {
    let min = state.iter().cloned().min().unwrap_or(0);
    let max = state.iter().cloned().max().unwrap_or(0);

    (min - 2..max + 3)
    .map(|i| (
        i,
        pattern_from_vec(
            &(i - 2..i + 3)
            .map(|j| state.contains(&j))
            .collect::<Vec<_>>()
        )
    ))
    .filter(|(i, pattern)|
        instructions.get(&pattern[..])
        .cloned()
        .unwrap_or_else(|| state.contains(&i))
    )
    .map(|(i, _)| i)
    .collect()
}

fn evolve(state: &State, instructions: &Instructions, n: usize) -> State {
    let next = evolve_gen(state, instructions);
    (1..n).fold(next, |state, _| evolve_gen(&state, instructions))
}

fn main() {
    let input = get_input().unwrap();
    let (state, instructions) = parse(&input);

    let sum = evolve(&state, &instructions, 20).iter()
        .cloned()
        .sum::<isize>();

    println!("Part 1: {}", sum);

    let ancient_gen = evolve(&state, &instructions, 1000);
    let next_gen_sum = evolve_gen(&ancient_gen, &instructions).iter()
        .cloned()
        .sum::<isize>();

    let diff = next_gen_sum - ancient_gen.iter()
        .cloned()
        .sum::<isize>();

    println!("Part 2: {}", (50000000000 - 1001) * diff + next_gen_sum);
}
