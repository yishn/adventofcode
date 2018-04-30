use std::fs::File;
use std::io::Read;

fn get_input() -> std::io::Result<String> {
    let mut file = File::open("17.txt")?;
    let mut contents = String::new();
    file.read_to_string(&mut contents)?;

    Ok(contents)
}

fn list_combinations(sum: u32, numbers: &Vec<u32>) -> Vec<Vec<u32>> {
    match (sum, numbers.len()) {
        (0, _) => vec![vec![]],
        (_, 0) => vec![],

        _ => {
            numbers.iter().cloned()
            .enumerate()
            .filter(|&(_, x)| x <= sum)
            .flat_map(|(i, x)| {
                list_combinations(
                    sum - x,
                    &numbers.iter().cloned()
                        .skip(i + 1)
                        .collect()
                ).into_iter().map(move |mut vec| {
                    vec.push(x);
                    vec
                })
            })
            .collect()
        }
    }
}

fn main() {
    let input = get_input().unwrap();
    let numbers = input.lines().filter_map(|line| match line.parse::<u32>() {
        Ok(x) => Some(x),
        Err(_) => None
    }).collect::<Vec<_>>();

    let combinations = list_combinations(150, &numbers);

    println!("Part 1: {}", combinations.len());

    let min_len = combinations.iter().map(|x| x.len()).min().unwrap();

    println!("Part 2: {}", combinations.iter().filter(|&x| x.len() == min_len).count());
}
