use std::fs::File;
use std::io::prelude::*;
use std::collections::HashSet;

fn get_input() -> std::io::Result<String> {
    let mut file = File::open("10.txt")?;
    let mut contents = String::new();
    file.read_to_string(&mut contents)?;

    Ok(contents)
}

#[derive(Debug)]
struct Point {
    pos: (isize, isize),
    vel: (isize, isize)
}

impl Point {
    fn new(x: isize, y: isize, dx: isize, dy: isize) -> Point {
        Point {pos: (x, y), vel: (dx, dy)}
    }

    fn ticks(&self, ticks: isize) -> Point {
        let &Point {pos: (x, y), vel: (dx, dy)} = self;
        Point::new(x + ticks * dx, y + ticks * dy, dx, dy)
    }
}

fn parse(input: &str) -> Vec<Point> {
    input.lines()
    .filter_map(|line| {
        let tokens: Vec<isize> = line
            .split(|c| c == '<' || c == '>' || c == ',')
            .filter_map(|token| token.trim().parse::<isize>().ok())
            .collect();

        if tokens.len() < 4 {
            None
        } else {
            Some(Point::new(tokens[0], tokens[1], tokens[2], tokens[3]))
        }
    })
    .collect()
}

fn draw_grid(points: &HashSet<(isize, isize)>) -> String {
    let minx = points.iter().map(|p| p.0).min().unwrap_or(0);
    let maxx = points.iter().map(|p| p.0).max().unwrap_or(0);
    let miny = points.iter().map(|p| p.1).min().unwrap_or(0);
    let maxy = points.iter().map(|p| p.1).max().unwrap_or(0);
    let mut result = String::new();

    for y in miny..maxy + 1 {
        for x in minx..maxx + 1 {
            if points.contains(&(x, y)) {
                result.push_str("# ");
            } else {
                result.push_str(". ");
            }
        }

        result.push('\n');
    }

    result.pop();
    result
}

fn main() {
    let input = get_input().unwrap();
    let points = parse(&input);

    let (t, grid) = (0..)
        .map(|t| {
            points.iter()
            .map(|p| p.ticks(t).pos)
            .collect::<HashSet<_>>()
        })
        .enumerate()
        .skip_while(|(_, points)| points.iter().any(|&(x, y)| {
            [
                (x - 1, y - 1),
                (x - 1, y),
                (x, y - 1),
                (x - 1, y + 1),
                (x + 1, y - 1),
                (x, y + 1),
                (x + 1, y),
                (x + 1, y + 1)
            ].iter().all(|p| !points.contains(p))
        }))
        .next()
        .map(|(t, points)| (t, draw_grid(&points)))
        .unwrap_or_else(|| (0, String::new()));

    println!("Part 1:\n{}", grid);
    println!("Part 2: {}", t);
}
