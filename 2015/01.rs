use std::fs::File;
use std::io::prelude::*;

fn get_input() -> std::io::Result<String> {
    let mut file = File::open("01.txt")?;
    let mut contents = String::new();
    file.read_to_string(&mut contents)?;

    Ok(contents)
}

fn main() {
    let input = get_input().unwrap();
    let mut floor = 0;
    let mut underground_index = -1;

    for (i, c) in input.chars().enumerate() {
        floor += match c {
            '(' => 1,
            ')' => -1,
            _ => 0
        };

        if floor < 0 && underground_index < 0 {
            underground_index = (i + 1) as i32;
        }
    }

    println!("Part 1: {}", floor);
    println!("Part 2: {}", underground_index);
}
