use std::fs::File;
use std::io::Read;
use std::collections::HashMap;

fn get_input() -> std::io::Result<String> {
    let mut file = File::open("16.txt")?;
    let mut contents = String::new();
    file.read_to_string(&mut contents)?;

    Ok(contents)
}

type Aunt<'a> = HashMap<&'a str, u32>;

fn parse_line(line: &str) -> Option<Aunt> {
    let colon_index = match line.find(|c| c == ':') {
        Some(x) => x,
        _ => return None
    };

    let data = line[colon_index + 2..]
        .split(", ")
        .filter_map(|s| {
            let data = s.split(": ").collect::<Vec<_>>();
            match (data.get(0), data.get(1)) {
                (Some(&x), Some(&y)) => match y.parse::<u32>() {
                    Ok(y) => Some((x, y)),
                    Err(_) => None
                },
                _ => None
            }
        });

    let mut aunt = Aunt::new();

    for (prop, value) in data {
        aunt.insert(prop, value);
    }

    Some(aunt)
}

fn main() {
    let input = get_input().unwrap();
    let aunts = input.lines().filter_map(parse_line).collect::<Vec<_>>();

    let analysis = [
        ("children", 3),
        ("cats", 7),
        ("samoyeds", 2),
        ("pomeranians", 3),
        ("akitas", 0),
        ("vizslas", 0),
        ("goldfish", 5),
        ("trees", 3),
        ("cars", 2),
        ("perfumes", 1)
    ].iter().cloned().collect::<HashMap<_, _>>();

    let aunt_number = aunts.iter().position(|aunt| {
        aunt.iter().all(|(&key, &value)| {
            analysis[key] == value
        })
    }).unwrap() + 1;

    println!("Part 1: {}", aunt_number);

    let aunt_number = aunts.iter().position(|aunt| {
        aunt.iter().all(|(&key, &value)| match key {
            "cats" | "trees" => analysis[key] < value,
            "pomeranians" | "goldfish" => analysis[key] > value,
            _ => analysis[key] == value
        })
    }).unwrap() + 1;

    println!("Part 2: {}", aunt_number);
}
