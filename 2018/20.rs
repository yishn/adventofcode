use std::fs::File;
use std::io::prelude::*;
use std::collections::{HashMap, HashSet, VecDeque};

fn get_input() -> std::io::Result<String> {
    let mut file = File::open("20.txt")?;
    let mut contents = String::new();
    file.read_to_string(&mut contents)?;

    Ok(contents)
}

type Point = (isize, isize);
type RoomMap = HashMap<Point, HashSet<Point>>;
type PathsMap = HashMap<Point, Option<Point>>;

fn parse(input: &str) -> Option<RoomMap> {
    let input = input.trim();
    let mut chars = input[1..input.len() - 1].chars();
    let mut result = RoomMap::new();

    fn inner(anchor: Point, chars: &mut Iterator<Item = char>, result: &mut RoomMap) -> Option<Point> {
        let mut current = anchor;

        while let Some(c) = chars.next() {
            let (x, y) = current;
            let next = match c {
                'N' => (x, y - 1),
                'W' => (x - 1, y),
                'E' => (x + 1, y),
                'S' => (x, y + 1),
                '|' => {
                    current = anchor;
                    continue;
                },
                '(' => {
                    current = match inner(current, chars, result) {
                        Some(p) => p,
                        _ => return None
                    };

                    continue;
                },
                ')' => break,
                _ => return None
            };

            if !result.contains_key(&current) {
                result.insert(current, HashSet::new());
            }

            if !result.contains_key(&next) {
                result.insert(next, HashSet::new());
            }

            result.get_mut(&current).unwrap().insert(next);
            result.get_mut(&next).unwrap().insert(current);

            current = next;
        }

        Some(current)
    }

    inner((0, 0), &mut chars, &mut result)
    .map(|_| result)
}

fn get_pathsmap_and_farthest_point(point: Point, map: &RoomMap) -> (PathsMap, Point) {
    let mut paths = HashMap::new();
    let mut queue = VecDeque::new();
    let mut farthest = point;

    queue.push_back(point);
    paths.insert(point, None);

    while let Some(p) = queue.pop_front() {
        farthest = p;

        let neighbors = match map.get(&p) {
            Some(x) => x,
            _ => continue
        };

        for &n in neighbors {
            if paths.contains_key(&n) {
                continue;
            }

            paths.insert(n, Some(p));
            queue.push_back(n);
        }
    }

    (paths, farthest)
}

fn path_from_pathsmap(target: Point, paths: &PathsMap) -> Vec<Point> {
    let mut current = target;
    let mut path = Vec::new();

    while let Some(&Some(prev)) = paths.get(&current) {
        path.push(prev);
        current = prev;
    }

    path.reverse();
    path
}

fn longest_path(point: Point, map: &RoomMap) -> Vec<Point> {
    let (paths, farthest) = get_pathsmap_and_farthest_point(point, map);
    path_from_pathsmap(farthest, &paths)
}

fn get_all_paths(point: Point, map: &RoomMap) -> Vec<Vec<Point>> {
    let (paths, _) = get_pathsmap_and_farthest_point(point, map);

    paths.keys()
    .map(|&target| path_from_pathsmap(target, &paths))
    .collect()
}

fn main() {
    let input = get_input().unwrap();
    let map = parse(&input).unwrap_or_else(|| RoomMap::new());

    println!("Part 1: {}", longest_path((0, 0), &map).len());

    let long_paths_count = get_all_paths((0, 0), &map).iter()
        .filter(|path| path.len() >= 1000)
        .count();

    println!("Part 2: {}", long_paths_count);
}
