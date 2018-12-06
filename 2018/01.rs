use std::fs::File;
use std::io::prelude::*;
use std::collections::HashSet;

fn get_input() -> std::io::Result<String> {
    let mut file = File::open("01.txt")?;
    let mut contents = String::new();
    file.read_to_string(&mut contents)?;

    Ok(contents)
}

fn main() {
    let input = get_input().unwrap();
    let data: Vec<i32> = input.lines().filter_map(|line| line.parse::<i32>().ok()).collect();

    let sum = data.iter().sum::<i32>();
    println!("Part 1: {}", sum);

    let mut sum = 0;
    let mut frequencies = HashSet::new();

    frequencies.insert(sum);

    for x in data.iter().cycle() {
        sum += x;

        if frequencies.contains(&sum) {
            break;
        }

        frequencies.insert(sum);
    }

    println!("Part 2: {}", sum);
}
