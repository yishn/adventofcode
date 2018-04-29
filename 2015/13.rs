use std::fs::File;
use std::io::Read;
use std::collections::HashMap;

fn get_input() -> std::io::Result<String> {
    let mut file = File::open("13.txt")?;
    let mut contents = String::new();
    file.read_to_string(&mut contents)?;

    Ok(contents)
}

struct Instruction<'a> {
    happiness: i32,
    name1: &'a str,
    name2: &'a str
}

type Names<'a> = Vec<&'a str>;
type HappinessIndex<'a> = HashMap<(&'a str, &'a str), i32>;

fn parse_line(line: &str) -> Option<Instruction> {
    let tokens = line.split(' ').collect::<Vec<_>>();
    if tokens.len() < 11 {
        return None;
    }

    let name1 = tokens[0];
    let name2 = tokens[tokens.len() - 1];
    let name2 = &name2[..name2.len() - 1];

    let sign = match tokens[2] {
        "gain" => 1,
        "lose" => -1,
        _ => return None
    };

    let value = match tokens[3].parse::<i32>() {
        Ok(x) => x,
        Err(_) => return None
    };

    let happiness = sign * value;

    Some(Instruction {name1, name2, happiness})
}

fn parse(input: &str) -> (Names, HappinessIndex) {
    let mut happiness_index = HashMap::new();
    let mut names = Vec::new();
    let instructions = input.lines().filter_map(parse_line);

    for Instruction {name1, name2, happiness} in instructions {
        if !names.contains(&name1) {
            names.push(name1);
        }

        if !names.contains(&name2) {
            names.push(name2);
        }

        happiness_index.insert((name1, name2), happiness);
    }

    (names, happiness_index)
}

fn get_happiness(happiness_index: &HappinessIndex, arrangement: &Names) -> i32 {
    let len = arrangement.len();

    arrangement.iter()
        .enumerate()
        .map(|(i, &name)| (name, arrangement[(i + 1) % len]))
        .map(|(x, y)| [(x, y), (y, x)])
        .map(|x| x.iter().map(|y| happiness_index.get(&y).unwrap()).sum::<i32>())
        .sum()
}

fn get_permutations<T: Clone>(list: &Vec<T>) -> Vec<Vec<T>> {
    let mut result = Vec::new();

    if list.len() == 0 {
        result.push(Vec::new());
        return result;
    }

    for i in 0..list.len() {
        let mut rest = list.clone();
        rest.remove(i);

        for mut permutation in get_permutations(&rest) {
            permutation.push(list[i].clone());
            result.push(permutation);
        }
    }

    result
}

fn get_max_happiness(names: &Names, happiness_index: &HappinessIndex) -> i32 {
    get_permutations(names).iter()
        .map(|permutation| get_happiness(happiness_index, &permutation))
        .max().unwrap()
}

fn main() {
    let input = get_input().unwrap();
    let (mut names, mut happiness_index) = parse(&input);

    let happiness = get_max_happiness(&names, &happiness_index);
    println!("Part 1: {}", happiness);

    for &name in names.iter() {
        happiness_index.insert((name, "Me"), 0);
        happiness_index.insert(("Me", name), 0);
    }

    names.push("Me");

    let happiness = get_max_happiness(&names, &happiness_index);
    println!("Part 2: {}", happiness);
}
