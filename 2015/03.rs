use std::fs::File;
use std::io::prelude::*;

fn get_input() -> std::io::Result<String> {
    let mut file = File::open("03.txt")?;
    let mut contents = String::new();
    file.read_to_string(&mut contents)?;

    Ok(contents)
}

#[derive(Debug, Copy, Clone, PartialEq)]
struct Pos(i32, i32);

fn get_direction(c: &char) -> Option<Pos> {
    match *c {
        '^' => Some(Pos(0, -1)),
        'v' => Some(Pos(0, 1)),
        '>' => Some(Pos(1, 0)),
        '<' => Some(Pos(-1, 0)),
        _ => None
    }
}

fn main() {
    let input = get_input().unwrap();
    let start = Pos(0, 0);

    let mut visited = Vec::new();
    let mut position = start;

    visited.push(start);

    for c in input.chars() {
        let (dx, dy) = match get_direction(&c) {
            Some(Pos(x, y)) => (x, y),
            None => continue
        };

        position.0 += dx;
        position.1 += dy;

        if !visited.contains(&position) {
            visited.push(position);
        }
    }

    println!("Part 1: {}", visited.len());

    // ---

    let mut visited = Vec::new();
    let mut santa_pos = start;
    let mut robo_pos = start;

    visited.push(start);

    for (i, c) in input.chars().enumerate() {
        let Pos(dx, dy) = match get_direction(&c) {
            Some(p) => p,
            None => continue
        };

        if i % 2 == 0 {
            santa_pos.0 += dx;
            santa_pos.1 += dy;

            if !visited.contains(&santa_pos) {
                visited.push(santa_pos);
            }
        } else {
            robo_pos.0 += dx;
            robo_pos.1 += dy;

            if !visited.contains(&robo_pos) {
                visited.push(robo_pos);
            }
        }
    }

    println!("Part 2: {}", visited.len());
}
