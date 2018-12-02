use std::fs::File;
use std::io::prelude::*;
use std::collections::HashMap;

fn get_input() -> std::io::Result<String> {
    let mut file = File::open("02.txt")?;
    let mut contents = String::new();
    file.read_to_string(&mut contents)?;

    Ok(contents)
}

fn get_char_histogram(input: &str) -> HashMap<char, usize> {
    let mut result = HashMap::new();

    for c in input.chars() {
        let v = match result.get(&c) {
            Some(&v) => v,
            None => 0
        };

        result.insert(c, v + 1);
    }

    result
}

fn get_checksum(ids: &Vec<String>) -> usize {
    let histograms: Vec<HashMap<char, usize>> = ids.iter()
        .map(|x| get_char_histogram(x))
        .collect();

    let exactly_two = histograms.iter()
        .filter(|hist| hist.iter().any(|(_, &v)| v == 2))
        .count();

    let exactly_three = histograms.iter()
        .filter(|hist| hist.iter().any(|(_, &v)| v == 3))
        .count();

    return exactly_two * exactly_three
}

fn diff_id(id1: &str, id2: &str) -> usize {
    id1.chars()
    .zip(id2.chars())
    .filter(|(c1, c2)| c1 != c2)
    .count()
}

fn find_correct_ids<'a>(ids: &'a Vec<String>) -> Option<(&'a str, &'a str)> {
    for (i, id1) in ids.iter().enumerate() {
        for id2 in ids.iter().skip(i + 1) {
            let diff = diff_id(id1, id2);

            if diff == 1 {
                return Some((id1, id2));
            }
        }
    }

    None
}

fn main() {
    let ids: Vec<String> = get_input().unwrap()
        .lines()
        .map(|x| x.to_string())
        .filter(|x| x != "")
        .collect();

    println!("Part 1: {}", get_checksum(&ids));

    if let Some((id1, id2)) = find_correct_ids(&ids) {
        let chars = id1.chars()
            .zip(id2.chars())
            .filter(|(c1, c2)| c1 == c2)
            .fold(String::new(), |acc, (c, _)| acc + &c.to_string());

        println!("Part 2: {}", chars);
    }
}
