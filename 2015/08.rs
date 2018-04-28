use std::fs::File;
use std::io::Read;

fn get_input() -> std::io::Result<String> {
    let mut file = File::open("08.txt")?;
    let mut contents = String::new();
    file.read_to_string(&mut contents)?;

    Ok(contents)
}

fn memory_length(literal: &str) -> i32 {
    let mut result = 0;
    let mut i = 1;
    let chars = literal.chars().collect::<Vec<_>>();

    while i < chars.len() - 1 {
        result += 1;

        i += match chars[i] {
            '\\' if chars[i + 1] == 'x' => 4,
            '\\' => 2,
            _ => 1
        };
    }

    result
}

fn literal_length(literal: &str) -> i32 {
    literal.chars().map(|c| match c {
        '\\' | '"' => 2,
        _ => 1
    }).sum::<i32>() + 2
}

fn main() {
    let input = get_input().unwrap();
    let lines = input.lines().collect::<Vec<_>>();
    let result = lines.iter()
        .map(|line| line.len() as i32 - memory_length(line))
        .sum::<i32>();

    println!("Part 1: {}", result);

    let result = lines.iter()
        .map(|line| literal_length(line) - line.len() as i32)
        .sum::<i32>();

    println!("Part 2: {}", result);
}
