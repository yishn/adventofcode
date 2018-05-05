use std::fs::File;
use std::io::Read;

fn get_input() -> std::io::Result<String> {
    let mut file = File::open("24.txt")?;
    let mut contents = String::new();
    file.read_to_string(&mut contents)?;

    Ok(contents)
}

fn list_subsets(numbers: &Vec<usize>, sum: usize, start_index: usize) -> Vec<Vec<usize>> {
    if sum == 0 {
        return vec![vec![]];
    } else if start_index >= numbers.len() {
        return vec![];
    }

    numbers
    .iter()
    .enumerate()
    .skip(start_index)
    .filter(|&(_, &x)| x <= sum)
    .flat_map(|(i, &x)| {
        list_subsets(numbers, sum - x, i + 1)
        .into_iter()
        .map(move |mut subset| {
            subset.push(x);
            subset
        })
    })
    .collect()
}

fn main() {
    let input = get_input().unwrap();
    let numbers = input.lines().filter_map(|line| match line.parse::<usize>() {
        Ok(x) => Some(x),
        Err(_) => None
    }).collect::<Vec<_>>();

    let bucket_size = numbers.iter().sum::<usize>() / 3;
    let buckets = list_subsets(&numbers, bucket_size, 0);

    let min_size = buckets.iter().map(|bucket| bucket.len()).min().unwrap();
    let qe = buckets.iter()
        .filter(|bucket| bucket.len() == min_size)
        .map(|bucket| bucket.into_iter().product::<usize>())
        .min().unwrap();

    println!("Part 1: {}", qe);

    let bucket_size = numbers.iter().sum::<usize>() / 4;
    let buckets = list_subsets(&numbers, bucket_size, 0);

    let min_size = buckets.iter().map(|bucket| bucket.len()).min().unwrap();
    let qe = buckets.iter()
        .filter(|bucket| bucket.len() == min_size)
        .map(|bucket| bucket.into_iter().product::<usize>())
        .min().unwrap();

    println!("Part 2: {}", qe);
}
