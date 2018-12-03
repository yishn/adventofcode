use std::fs::File;
use std::io::prelude::*;
use std::collections::{HashMap, HashSet};

fn get_input() -> std::io::Result<String> {
    let mut file = File::open("03.txt")?;
    let mut contents = String::new();
    file.read_to_string(&mut contents)?;

    Ok(contents)
}

type Fabric = HashMap<(usize, usize), Vec<usize>>;

#[derive(Debug)]
struct Rect {
    id: usize,
    left: usize,
    top: usize,
    width: usize,
    height: usize
}

fn parse_input(input: &str) -> Vec<Rect> {
    input.lines()
    .filter_map(|line| {
        let tokens: Vec<&str> = line.split(' ').collect();

        if tokens.len() < 4 {
            return None;
        }

        let mut position = tokens[2][..tokens[2].len() - 1]
            .split(',')
            .filter_map(|x| x.parse::<usize>().ok());

        let mut size = tokens[3]
            .split('x')
            .filter_map(|x| x.parse::<usize>().ok());

        Some(Rect {
            id: match tokens[0][1..].parse::<usize>() {
                Ok(x) => x,
                Err(_) => return None
            },
            left: match position.next() {
                Some(x) => x,
                None => return None
            },
            top: match position.next() {
                Some(x) => x,
                None => return None
            },
            width: match size.next() {
                Some(x) => x,
                None => return None
            },
            height: match size.next() {
                Some(x) => x,
                None => return None
            }
        })
    })
    .collect()
}

fn reserve_fabric(fabric: &mut Fabric, rect: &Rect, no_overlaps: &mut HashSet<usize>) {
    let mut overlap = false;

    for x in rect.left..rect.left + rect.width {
        for y in rect.top..rect.top + rect.height {
            if fabric.contains_key(&(x, y)) {
                let mut ids = fabric.get_mut(&(x, y)).unwrap();

                if ids.len() == 1 {
                    let id = ids[0];
                    no_overlaps.remove(&id);
                }

                overlap = true;
                ids.push(rect.id);
            } else {
                fabric.insert((x, y), vec![rect.id]);
            }
        }
    }

    if !overlap {
        no_overlaps.insert(rect.id);
    } else {
        no_overlaps.remove(&rect.id);
    }
}

fn main() {
    let input = get_input().unwrap();
    let rects = parse_input(&input);

    let mut fabric = Fabric::new();
    let mut no_overlaps = HashSet::new();

    for rect in rects.iter() {
        reserve_fabric(&mut fabric, rect, &mut no_overlaps);
    }

    let overlapped = fabric.iter()
        .filter(|(_, v)| v.len() >= 2)
        .count();

    println!("Part 1: {}", overlapped);
    println!("Part 2: {}", match no_overlaps.iter().next() {
        Some(x) => x,
        None => return
    });
}
