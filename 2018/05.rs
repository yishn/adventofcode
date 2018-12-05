use std::fs::File;
use std::io::prelude::*;

fn get_input() -> std::io::Result<String> {
    let mut file = File::open("05.txt")?;
    let mut contents = String::new();
    file.read_to_string(&mut contents)?;

    Ok(contents)
}

fn react<T>(chars: T) -> String
where T: Iterator<Item = char> {
    chars.fold(String::new(), |mut acc, c| {
        match acc.chars().rev().next() {
            Some(d) if d != c && d.eq_ignore_ascii_case(&c) => { acc.pop(); },
            _ => acc.push(c)
        };

        acc
    })
}

fn main() {
    let input = get_input().unwrap();
    let polymer = input.trim();
    let reacted = react(polymer.chars());

    println!("Part 1: {}", reacted.len());

    let improved = "abcdefghijklmnopqrstuvwxyz".chars()
        .map(|c| react(polymer.chars().filter(|d| !d.eq_ignore_ascii_case(&c))))
        .map(|reacted| reacted.len())
        .min()
        .unwrap();

    println!("Part 2: {}", improved);
}
