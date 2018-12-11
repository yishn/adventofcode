use std::fs::File;
use std::io::prelude::*;

const SIZE: usize = 300;

fn get_input() -> std::io::Result<String> {
    let mut file = File::open("11.txt")?;
    let mut contents = String::new();
    file.read_to_string(&mut contents)?;

    Ok(contents)
}

fn coord_to_index((x, y): (usize, usize)) -> usize {
    (x - 1) + (y - 1) * SIZE
}

fn get_largest_fixed_square(partial_sums: &Vec<isize>, square: usize) -> Option<((usize, usize), isize)> {
    (1..SIZE + 2 - square)
    .flat_map(|x| (1..SIZE + 2 - square).map(move |y| (x, y)))
    .map(|(x, y)| (
        (x, y),
        [
            (x + square - 1, y + square - 1),
            (x + square - 1, y - 1),
            (x - 1, y - 1),
            (x - 1, y + square - 1)
        ]
        .iter()
        .cloned()
        .enumerate()
        .filter(|&(_, (x, y))| 1 <= x && x <= SIZE && 1 <= y && y <= SIZE)
        .map(|(i, coord)| (i, coord_to_index(coord)))
        .map(|(i, j)| (-1isize).pow(i as u32) * partial_sums.get(j).cloned().unwrap_or(0))
        .sum::<isize>()
    ))
    .max_by_key(|&(_, power)| power)
}

fn main() {
    let serial_number = get_input().unwrap().trim().parse::<usize>().unwrap();
    let fuel_cells: Vec<isize> = (1..SIZE + 1)
        .flat_map(|y| (1..SIZE + 1).map(move |x| (x, y)))
        .map(|(x, y)| {
            let rack_id = x + 10;
            ((rack_id * y + serial_number) * rack_id % 1000) as isize / 100 - 5
        })
        .collect();

    let mut partial_sums = Vec::new();

    for (i, power) in fuel_cells.iter().cloned().enumerate() {
        let x = i % SIZE;
        let sum = power
            + if i >= SIZE { partial_sums[i - SIZE] } else { 0 }
            + if x > 0 { partial_sums[i - 1] } else { 0 }
            - if x > 0 && i >= SIZE + 1 { partial_sums[i - SIZE - 1] } else { 0 };

        partial_sums.push(sum);
    }

    get_largest_fixed_square(&partial_sums, 3)
    .map(|((x, y), _)| {
        println!("Part 1: {},{}", x, y)
    });

    (1..SIZE + 1)
    .filter_map(|square_size| {
        get_largest_fixed_square(&partial_sums, square_size)
        .map(|x| (x, square_size))
    })
    .max_by_key(|&((_, power), _)| power)
    .map(|(((x, y), _), square_size)| {
        println!("Part 2: {},{},{}", x, y, square_size)
    });
}
