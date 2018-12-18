use std::cmp::max;
use std::fs::File;
use std::io::prelude::*;
use std::collections::{HashSet, HashMap};

fn get_input() -> std::io::Result<String> {
    let mut file = File::open("17.txt")?;
    let mut contents = String::new();
    file.read_to_string(&mut contents)?;

    Ok(contents)
}

type Point = (isize, isize);
type DepthMap = HashSet<Point>;

fn parse(input: &str) -> DepthMap {
    input.lines()
    .flat_map(|line| {
        let parts = line.split(", ")
            .filter_map(|part| {
                let mut tokens = part.split('=');

                match (tokens.next(), tokens.next()) {
                    (Some(coord), Some(value)) => Some((coord, value)),
                    _ => None
                }
            })
            .map(|(coord, value)| (
                coord,
                match value.parse::<usize>() {
                    Ok(x) => vec![x, x],
                    _ => value.split("..").filter_map(|x| x.parse::<usize>().ok()).collect()
                }
            ))
            .filter(|(_, values)| values.len() == 2)
            .fold(HashMap::new(), |mut acc, (coord, values)| {
                acc.insert(coord, values);
                acc
            });

        let (xstart, xend) = parts.get(&"x")
            .map(|v| (v[0], v[1]))
            .unwrap_or((0, 0));

        let (ystart, yend) = parts.get(&"y")
            .map(|v| (v[0], v[1]))
            .unwrap_or((0, 0));

        (xstart..xend + 1)
        .flat_map(move |x| (ystart..yend + 1).map(move |y| (x as isize, y as isize)))
    })
    .collect()
}

fn fill_water(source: Point, map: &DepthMap) -> (DepthMap, DepthMap) {
    let miny = map.iter().cloned().map(|(_, y)| y).min().unwrap_or(source.1);
    let maxy = map.iter().cloned().map(|(_, y)| y).max().unwrap_or(source.1);

    let mut water = DepthMap::new();
    let mut flows = DepthMap::new();
    let mut blocked = map.clone();

    fn flows_down((x, y): Point, blocked: &DepthMap) -> bool {
        !blocked.contains(&(x, y + 1))
    }

    fn inner(
        source: Point,
        map: &DepthMap,
        water: &mut DepthMap,
        flows: &mut DepthMap,
        blocked: &mut DepthMap,
        miny: isize,
        maxy: isize
    ) {
        let (sx, sy) = source;

        if water.contains(&source) {
            return;
        }

        let (ax, ay) = (sy..maxy + 1)
            .map(|y| (sx, y))
            .find(|&p| !flows_down(p, &blocked))
            .unwrap_or((sx, maxy));

        let infinite = ay == maxy;

        let ((lx, ly), (rx, ry)) = {
            let left = (0..)
                .map(|x| (ax - x, ay))
                .find(|&(x, y)| {
                    blocked.contains(&(x - 1, y))
                    || flows_down((x, y), &blocked)
                })
                .unwrap();

            let right = (0..)
                .map(|x| (ax + x, ay))
                .find(|&(x, y)| {
                    blocked.contains(&(x + 1, y))
                    || flows_down((x, y), &blocked)
                })
                .unwrap();

            (left, right)
        };

        // Update flows & water

        let left_overflow = flows_down((lx, ly), &blocked);
        let right_overflow = flows_down((rx, ry), &blocked);
        let overflow = left_overflow || right_overflow;
        let top = (lx..rx + 1).map(|x| (x, ay)).collect::<DepthMap>();
        let old_flow_count = flows.len();

        if !overflow {
            *water = water.union(&top).cloned().collect();
            *blocked = blocked.union(&top).cloned().collect();
        } else {
            *flows = flows
                .union(&top)
                .cloned()
                .chain(
                    if !flows.contains(&(sx, sy + 1)) {
                        (max(sy, miny)..ay + 1)
                    } else {
                        (0..0)
                    }
                    .map(|y| (sx, y))
                )
                .collect();

            if old_flow_count == flows.len() {
                return;
            }
        }

        // Propagate

        let (left_source, right_source) = (
            left_overflow && (lx, ly) != (ax, ay),
            right_overflow && (rx, ry) != (ax, ay)
        );

        if left_source {
            inner((lx, ly), map, water, flows, blocked, miny, maxy);
        }

        if right_source {
            inner((rx, ry), map, water, flows, blocked, miny, maxy);
        }

        if !infinite {
            inner(source, map, water, flows, blocked, miny, maxy);
        }
    }

    inner(source, map, &mut water, &mut flows, &mut blocked, miny, maxy);

    (water, flows)
}

fn main() {
    let input = get_input().unwrap();
    let depth_map = parse(&input);
    let (water, flows) = fill_water((500, 0), &depth_map);

    println!("Part 1: {}", water.union(&flows).count());
    println!("Part 2: {}", water.len());
}
