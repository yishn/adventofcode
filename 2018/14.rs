use std::fs::File;
use std::io::prelude::*;

fn get_input() -> std::io::Result<String> {
    let mut file = File::open("14.txt")?;
    let mut contents = String::new();
    file.read_to_string(&mut contents)?;

    Ok(contents)
}

fn combine_recipes(scores: &mut Vec<usize>, elf1: &mut usize, elf2: &mut usize) {
    let score1 = scores.get(*elf1).cloned().unwrap_or(0);
    let score2 = scores.get(*elf2).cloned().unwrap_or(0);
    let score_sum = (score1 + score2).to_string();
    let new_scores = score_sum.chars()
        .filter_map(|c| c.to_digit(10))
        .map(|x| x as usize);

    scores.extend(new_scores);

    *elf1 = (*elf1 + 1 + score1) % scores.len();
    *elf2 = (*elf2 + 1 + score2) % scores.len();
}

fn main() {
    let input = get_input().ok()
        .and_then(|x| x.trim().parse::<usize>().ok())
        .unwrap_or(0);

    let (mut scores, mut elf1, mut elf2) = (vec![3, 7], 0, 1);

    while scores.len() < input + 10 {
        combine_recipes(&mut scores, &mut elf1, &mut elf2);
    }

    let result = scores[input..input + 10].iter()
        .fold(String::new(), |acc, x| acc + &x.to_string());

    println!("Part 1: {}", result);

    let find_scores = get_input().unwrap().to_string()
        .chars()
        .filter_map(|c| c.to_digit(10))
        .map(|x| x as usize)
        .collect::<Vec<_>>();

    let (mut scores, mut elf1, mut elf2) = (vec![3, 7], 0, 1);
    let mut found = None;
    let mut check_index = 0;

    while found.is_none() {
        combine_recipes(&mut scores, &mut elf1, &mut elf2);

        if scores.len() > find_scores.len() {
            found = (check_index..scores.len() - find_scores.len()).find(|&i| {
                scores[i..i + find_scores.len()].iter()
                .enumerate()
                .all(|(j, &x)| x == find_scores[j])
            });

            check_index = scores.len() - find_scores.len();
        }
    }

    println!("Part 2: {}", found.unwrap());
}
