use std::fs::File;
use std::io::prelude::*;

fn get_input() -> std::io::Result<String> {
    let mut file = File::open("02.txt")?;
    let mut contents = String::new();
    file.read_to_string(&mut contents)?;

    Ok(contents)
}

fn main() {
    let input = get_input().unwrap();
    let lines = input.split('\n').map(|x| x.replace("\r", ""));

    let mut area = 0;
    let mut ribbon = 0;

    for line in lines {
        let mut sides: Vec<i32> = line.split('x').filter_map(|x| match x.parse::<i32>() {
            Ok(y) => Some(y),
            Err(_) => None
        }).collect();

        let side_areas = match (sides.get(0), sides.get(1), sides.get(2)) {
            (Some(l), Some(w), Some(h)) => [l * w, w * h, h * l],
            _ => continue
        };

        sides.sort();

        area += 2 * side_areas.iter().sum::<i32>() + side_areas.iter().min().unwrap();
        ribbon += 2 * sides.iter().take(2).sum::<i32>() + sides.iter().product::<i32>();
    }

    println!("Part 1: {}", area);
    println!("Part 2: {}", ribbon);
}
