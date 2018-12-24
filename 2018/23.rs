use std::fs::File;
use std::io::prelude::*;
use std::collections::BinaryHeap;

fn get_input() -> std::io::Result<String> {
    let mut file = File::open("23.txt")?;
    let mut contents = String::new();
    file.read_to_string(&mut contents)?;

    Ok(contents)
}

type Cube = (isize, isize, isize, isize, isize, isize);
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

fn subdivide_cube((xmin, xmax, ymin, ymax, zmin, zmax): Cube) -> Vec<Cube> {
    let xmid = (xmin + xmax) / 2;
    let ymid = (ymin + ymax) / 2;
    let zmid = (zmin + zmax) / 2;

    vec![
        (xmin, xmid, ymin, ymid, zmin, zmid),
        (xmin, xmid, ymin, ymid, zmid + 1, zmax),
        (xmin, xmid, ymid + 1, ymax, zmin, zmid),
        (xmin, xmid, ymid + 1, ymax, zmid + 1, zmax),
        (xmid + 1, xmax, ymin, ymid, zmin, zmid),
        (xmid + 1, xmax, ymin, ymid, zmid + 1, zmax),
        (xmid + 1, xmax, ymid + 1, ymax, zmin, zmid),
        (xmid + 1, xmax, ymid + 1, ymax, zmid + 1, zmax)
    ].into_iter()
    .filter(|&(xmin, xmax, ymin, ymax, zmin, zmax)| xmin <= xmax && ymin <= ymax && zmin <= zmax)
    .collect()
}

fn cube_to_range((xmin, xmax, ymin, ymax, zmin, zmax): Cube) -> Nanobot {
    let xmid = (xmin + xmax) / 2;
    let ymid = (ymin + ymax) / 2;
    let zmid = (zmin + zmax) / 2;
    let position = (xmid, ymid, zmid);
    let radius = [
        manhattan_dist(position, (xmin, ymin, zmin)),
        manhattan_dist(position, (xmin, ymin, zmax)),
        manhattan_dist(position, (xmin, ymax, zmin)),
        manhattan_dist(position, (xmin, ymax, zmax)),
        manhattan_dist(position, (xmax, ymin, zmin)),
        manhattan_dist(position, (xmax, ymin, zmax)),
        manhattan_dist(position, (xmax, ymax, zmin)),
        manhattan_dist(position, (xmax, ymax, zmax))
    ].into_iter().cloned().min().unwrap();

    Nanobot {
        position,
        radius
    }
}

fn is_position((xmin, xmax, ymin, ymax, zmin, zmax): Cube) -> bool {
    xmin == xmax && ymin == ymax && zmin == zmax
}

fn bots_near_cube(bots: &[Nanobot], cube: Cube) -> Vec<&Nanobot> {
    let range = cube_to_range(cube);

    bots.iter()
    .filter(|&bot| has_range_overlap(bot, &range))
    .collect()
}

fn find_position(bots: &[Nanobot]) -> Option<Position> {
    let mut heap = BinaryHeap::new();
    let mut max = None;

    let start_cube = {
        let xmin = bots.iter().map(|bot| bot.position.0).min().unwrap_or(0);
        let xmax = bots.iter().map(|bot| bot.position.0).max().unwrap_or(0);
        let ymin = bots.iter().map(|bot| bot.position.1).min().unwrap_or(0);
        let ymax = bots.iter().map(|bot| bot.position.1).max().unwrap_or(0);
        let zmin = bots.iter().map(|bot| bot.position.2).min().unwrap_or(0);
        let zmax = bots.iter().map(|bot| bot.position.2).max().unwrap_or(0);

        (xmin, xmax, ymin, ymax, zmin, zmax)
    };

    heap.push((bots.len(), start_cube));

    while let Some((bots_count, cube)) = heap.pop() {
        if is_position(cube) {
            let position = (cube.0, cube.2, cube.4);
            let distance = manhattan_dist(position, (0, 0, 0)) as isize;

            if max.is_none() || max.unwrap() < (bots_count, -distance, position) {
                max = Some((bots_count, -distance, position));
            }
        }

        for subcube in subdivide_cube(cube).into_iter() {
            let bots_count = bots_near_cube(bots, subcube).len();

            if max.is_none() || bots_count > max.unwrap().0 {
                heap.push((bots_count, subcube))
            }
        }
    }

    max.map(|(_, _, p)| p)
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

    find_position(&nanobots)
    .map(|p| println!("Part 2: {}", manhattan_dist(p, (0, 0, 0))));
}
