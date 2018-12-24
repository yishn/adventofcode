use std::fs::File;
use std::io::prelude::*;
use std::collections::{HashMap, HashSet};

fn get_input() -> std::io::Result<String> {
    let mut file = File::open("23.txt")?;
    let mut contents = String::new();
    file.read_to_string(&mut contents)?;

    Ok(contents)
}

type Position = (isize, isize, isize);

#[derive(Debug, Clone, Eq, PartialEq)]
struct Nanobot {
    position: Position,
    radius: usize
}

fn parse(input: &str) -> Vec<Nanobot> {
    input.lines()
    .map(|line| line.split(|c| ['<', '>', ',', '='].contains(&c)))
    .map(|mut tokens| [tokens.nth(2), tokens.next(), tokens.next(), tokens.nth(2)])
    .map(|tokens| {
        tokens.into_iter()
        .filter_map(|token| token.and_then(|x| x.parse::<isize>().ok()))
        .collect::<Vec<_>>()
    })
    .filter(|tokens| tokens.len() == 4)
    .map(|tokens| Nanobot {
        position: (tokens[0], tokens[1], tokens[2]),
        radius: tokens[3] as usize
    })
    .collect()
}

fn manhattan_dist((x1, y1, z1): Position, (x2, y2, z2): Position) -> usize {
    ((x2 - x1).abs() + (y2 - y1).abs() + (z2 - z1).abs()) as usize
}

fn in_bot_range(bot: &Nanobot, p: Position) -> bool {
    manhattan_dist(bot.position, p) <= bot.radius
}

fn has_range_overlap(bot1: &Nanobot, bot2: &Nanobot) -> bool {
    manhattan_dist(bot1.position, bot2.position) <= bot1.radius + bot2.radius
}

fn main() {
    let input = get_input().unwrap();
    let nanobots = parse(&input);

    nanobots.iter()
    .max_by_key(|bot| bot.radius)
    .map(|strongest| {
        nanobots.iter()
        .filter(|bot| in_bot_range(strongest, bot.position))
        .count()
    })
    .map(|count| println!("Part 1: {}", count));
}
