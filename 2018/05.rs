use std::fs::File;
use std::io::prelude::*;

fn get_input() -> std::io::Result<String> {
    let mut file = File::open("05.txt")?;
    let mut contents = String::new();
    file.read_to_string(&mut contents)?;

    Ok(contents)
}

fn react(input: &str) -> String {
    input.chars()
    .fold(String::new(), |mut acc, c| {
        match acc.chars().rev().next() {
            Some(d) if d.eq_ignore_ascii_case(&c) && d != c => { acc.pop(); },
            _ => acc.push(c)
        };

        acc
    })
}

fn main() {
    let input = get_input().unwrap();
    let reacted = react(input.trim());

    println!("Part 1: {}", reacted.len());

    let improved = "abcdefghijklmnopqrstuvwxyz".chars()
        .map(|c| {
            let mut polymer = input.clone();

            polymer.retain(|d| !d.eq_ignore_ascii_case(&c));
            react(&polymer)
        })
        .map(|reacted| reacted.len())
        .min()
        .unwrap();

    println!("Part 2: {}", improved);
}
