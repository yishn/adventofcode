use std::fs::File;
use std::io::Read;

fn get_input() -> std::io::Result<String> {
    let mut file = File::open("05.txt")?;
    let mut contents = String::new();
    file.read_to_string(&mut contents)?;

    Ok(contents)
}

fn nice_string(string: &String) -> bool {
    let vowels: Vec<char> = "aeiou".chars().collect();

    // Contains at least three vowels
    string.chars().filter(|c| vowels.contains(c)).count() >= 3

    // Contains at least one letter that appears twice in a row
    && string.chars().any(|c| string.contains(&format!("{}{}", c, c)))

    // Does not contain certain strings
    && ["ab", "cd", "pq", "xy"].into_iter().all(|s| !string.contains(s))
}

fn nice_string_2(string: &String) -> bool {
    // Contains at least one letter which repeats with one letter between them
    string.len() >= 3 && (0..string.len() - 2)
        .map(|i| (&string[i..i + 3]).chars())
        .any(|mut aba| aba.nth(0).unwrap() == aba.nth(1).unwrap())

    // Contains a pair of any two letters that appears at least twice without overlapping
    && string.len() >= 4 && (0..string.len() - 3)
        .map(|i| (i + 2, &string[i..i + 2]))
        .any(|(i, xy)| (i..string.len() - 1).any(|j| &string[j..j + 2] == xy))
}

fn main() {
    let input = get_input().unwrap();
    let lines: Vec<String> = input.split('\n').map(|x| x.replace("\r", "")).collect();
    let nice_count = lines.iter().filter(|&s| nice_string(s)).count();
    let nice_count_2 = lines.iter().filter(|&s| nice_string_2(s)).count();

    println!("Part 1: {}", nice_count);
    println!("Part 2: {}", nice_count_2);
}
