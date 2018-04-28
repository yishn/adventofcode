use std::fs::File;
use std::io::Read;
use std::collections::HashMap;

fn get_input() -> std::io::Result<String> {
    let mut file = File::open("09.txt")?;
    let mut contents = String::new();
    file.read_to_string(&mut contents)?;

    Ok(contents)
}

type Cities<'a> = Vec<&'a str>;
type Distances<'a> = HashMap<(&'a str, &'a str), u32>;

fn parse<'a, T: Iterator<Item = &'a str>>(lines: T) -> (Cities<'a>, Distances<'a>) {
    let mut cities = Vec::new();
    let mut distances = HashMap::new();

    for line in lines {
        let tokens = line.split(" = ").collect::<Vec<_>>();
        if tokens.len() < 2 {
            continue;
        }

        let distance = match tokens[1].parse::<u32>() {
            Ok(x) => x,
            Err(_) => continue
        };

        let city_tokens = tokens[0].split(" to ").collect::<Vec<_>>();
        if city_tokens.len() < 2 {
            continue;
        }

        if !cities.contains(&city_tokens[0]) {
            cities.push(city_tokens[0]);
        }

        if !cities.contains(&city_tokens[1]) {
            cities.push(city_tokens[1]);
        }

        distances.insert((city_tokens[0], city_tokens[1]), distance);
        distances.insert((city_tokens[1], city_tokens[0]), distance);
    }

    (cities, distances)
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

fn get_distance(distances: &Distances, path: &Cities) -> u32 {
    path.iter().enumerate().skip(1)
        .map(|(i, city)| distances.get(&(city, path[i - 1])).unwrap())
        .sum::<u32>()
}

fn main() {
    let input = get_input().unwrap();
    let (cities, distances) = parse(input.lines());
    let permutations = get_permutations(&cities);
    let path_distances = permutations.iter()
        .map(|path| get_distance(&distances, &path))
        .collect::<Vec<_>>();

    println!("Part 1: {}", path_distances.iter().min().unwrap());
    println!("Part 2: {}", path_distances.iter().max().unwrap());
}
