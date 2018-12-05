use std::fs::File;
use std::io::prelude::*;
use std::collections::HashMap;

fn get_input() -> std::io::Result<String> {
    let mut file = File::open("05.txt")?;
    let mut contents = String::new();
    file.read_to_string(&mut contents)?;

    Ok(contents)
}

fn react(input: &str) -> String {
    let mut result;
    let mut polymer = input.to_string();

    loop {
        let chars: Vec<char> = polymer.chars().collect();
        let mut ignore_index = None;

        result = String::new();

        for (i, c) in chars.iter().enumerate() {
            if ignore_index == Some(i) {
                continue;
            }

            if i + 1 < chars.len() {
                let next_char = chars[i + 1];
                let reactable = next_char.eq_ignore_ascii_case(c) && match c.is_ascii_uppercase() {
                    true => next_char.is_ascii_lowercase(),
                    false => next_char.is_ascii_uppercase()
                };

                if reactable {
                    ignore_index = Some(i + 1);
                    continue;
                }
            }

            result.push(*c);
        }

        if result == polymer {
            return result;
        } else {
            polymer = result;
        }
    }
}

fn main() {
    let input = get_input().unwrap();
    let polymer = input.trim().to_string();
    let reacted = react(&polymer);

    println!("Part 1: {}", reacted.len());

    let improved = "abcdefghijklmnopqrstuvwxyz".chars()
        .map(|c| {
            polymer.chars()
            .filter(|&d| d != c && d != c.to_ascii_uppercase())
            .fold(String::new(), |mut acc, d| {
                acc.push(d);
                acc
            })
        })
        .map(|polymer| react(&polymer))
        .map(|reacted| reacted.len())
        .min()
        .unwrap();

    println!("Part 2: {}", improved);
}
