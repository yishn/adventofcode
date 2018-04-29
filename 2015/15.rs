use std::fs::File;
use std::io::Read;

fn get_input() -> std::io::Result<String> {
    let mut file = File::open("15.txt")?;
    let mut contents = String::new();
    file.read_to_string(&mut contents)?;

    Ok(contents)
}

struct Ingredient {
    capacity: i32,
    durability: i32,
    flavor: i32,
    texture: i32,
    calories: i32
}

fn parse_line(line: &str) -> Option<Ingredient> {
    let mut data = match line.split(": ").nth(1) {
        Some(x) => x,
        _ => return None
    }.split(", ").filter_map(|x| {
        x.split(' ').nth(1)
    }).filter_map(|x| match x.parse::<i32>() {
        Ok(x) => Some(x),
        Err(_) => None
    });

    let capacity = data.next().unwrap();
    let durability = data.next().unwrap();
    let flavor = data.next().unwrap();
    let texture = data.next().unwrap();
    let calories = data.next().unwrap();

    Some(Ingredient {capacity, durability, flavor, texture, calories})
}

fn get_score(
    ingredients: &Vec<Ingredient>,
    partition: &Vec<i32>,
    calory_limit: Option<i32>
) -> i32 {
    if ingredients.len() != partition.len() {
        return 0;
    }

    let capacity = ingredients.iter().enumerate()
        .map(|(i, x)| partition[i] * x.capacity).sum::<i32>();
    let durability = ingredients.iter().enumerate()
        .map(|(i, x)| partition[i] * x.durability).sum::<i32>();
    let flavor = ingredients.iter().enumerate()
        .map(|(i, x)| partition[i] * x.flavor).sum::<i32>();
    let texture = ingredients.iter().enumerate()
        .map(|(i, x)| partition[i] * x.texture).sum::<i32>();
    let calories = ingredients.iter().enumerate()
        .map(|(i, x)| partition[i] * x.calories).sum::<i32>();

    match calory_limit {
        Some(x) if calories > x => 0,
        _ => {
            if [capacity, durability, flavor, texture].iter().any(|&x| x < 0) {
                return 0;
            }

            capacity * durability * flavor * texture
        }
    }
}

fn list_partitions(total: i32, parts: u32) -> Vec<Vec<i32>> {
    match parts {
        0 => vec![vec![]],
        1 => vec![vec![total]],
        _ => {
            (0..total + 1).flat_map(|i| {
                list_partitions(total - i, parts - 1)
                .into_iter()
                .map(move |mut partition| {
                    partition.push(i);
                    partition
                })
            }).collect::<Vec<_>>()
        }
    }
}

fn main() {
    let input = get_input().unwrap();
    let ingredients = input.lines().filter_map(parse_line).collect::<Vec<_>>();
    let partitions = list_partitions(100, ingredients.len() as u32);

    let max_score = partitions.iter()
        .map(|partition| get_score(&ingredients, partition, None))
        .max().unwrap();

    println!("Part 1: {}", max_score);

    let max_score = partitions.iter()
        .map(|partition| get_score(&ingredients, partition, Some(500)))
        .max().unwrap();

    println!("Part 2: {}", max_score);
}
