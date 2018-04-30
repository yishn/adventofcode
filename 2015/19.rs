use std::fs::File;
use std::io::Read;

fn get_input() -> std::io::Result<String> {
    let mut file = File::open("19.txt")?;
    let mut contents = String::new();
    file.read_to_string(&mut contents)?;

    Ok(contents)
}

type Replacements<'a> = Vec<(&'a str, &'a str)>;

fn parse<'a>(input: &'a str) -> (&'a str, Replacements<'a>) {
    let mut lines = input.lines().filter(|&line| line.len() > 0).collect::<Vec<_>>();
    let mut replacements = Replacements::new();
    let start_string = lines.pop().unwrap();

    for line in lines {
        let mut tokens = line.split(" => ");

        if let (Some(key), Some(value)) = (tokens.next(), tokens.next()) {
            replacements.push((key, value));
        }
    }

    (start_string, replacements)
}

fn list_replacements(replacements: &Replacements, string: &str) -> Vec<String> {
    let mut result = replacements.iter().flat_map(|&(key, value)| {
        let l = key.len();

        (0..string.len()).filter_map(move |i| {
            if i + l <= string.len() && &string[i..i + l] == key {
                Some(String::new() + &string[..i] + value + &string[i + l..])
            } else {
                None
            }
        })
    }).collect::<Vec<_>>();

    result.sort();
    result.dedup();
    result
}

fn tokenize(string: &str) -> Vec<&str> {
    string.chars()
    .enumerate()
    .filter(|&(_, c)| c.to_uppercase().to_string() == c.to_string())
    .map(|(i, _)| match string.chars().nth(i + 1) {
        Some(x) if x.to_uppercase().to_string() != x.to_string() => {
            &string[i..i + 2]
        },
        _ => {
            &string[i..i + 1]
        }
    })
    .collect()
}

fn main() {
    let input = get_input().unwrap();
    let (input, replacements) = parse(&input);
    let list = list_replacements(&replacements, &input);

    println!("Part 1: {}", list.len());

    let tokens = tokenize(&input);
    let result = tokens.len()
        - tokens.iter().filter(|&&x| x == "Rn" || x == "Ar").count()
        - tokens.iter().filter(|&&x| x == "Y").count() * 2 - 1;

    println!("Part 2: {}", result);
}
