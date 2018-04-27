use std::fs::File;
use std::io::Read;

fn get_input() -> std::io::Result<String> {
    let mut file = File::open("06.txt")?;
    let mut contents = String::new();
    file.read_to_string(&mut contents)?;

    Ok(contents)
}

#[derive(Debug, Copy, Clone, PartialEq)]
enum Action {Toggle, TurnOff, TurnOn}

#[derive(Debug, Copy, Clone, PartialEq)]
struct Point(usize, usize);

#[derive(Debug)]
struct Instruction {
    action: Action,
    point1: Point,
    point2: Point
}

fn parse_point(string: &str) -> Option<Point> {
    let mut coords = string.split(',')
        .map(|x| x.parse::<usize>())
        .map(|x| match x {
            Ok(y) => Some(y),
            Err(_) => None
        });

    match (coords.next(), coords.next()) {
        (Some(Some(x)), Some(Some(y))) if x < 1000 && y < 1000 => {
            Some(Point(x, y))
        },
        _ => return None
    }
}

fn parse_line(line: &str) -> Option<Instruction> {
    let words: Vec<&str> = line.split(' ').collect();
    let action = match words.len() {
        l if l == 4 => {
            Action::Toggle
        },
        l if l == 5 && *words.get(1).unwrap() == "off" => {
            Action::TurnOff
        },
        l if l == 5 && *words.get(1).unwrap() == "on" => {
            Action::TurnOn
        },
        _ => return None
    };

    let point1 = parse_point(*words.get(words.len() - 3).unwrap());
    let point2 = parse_point(*words.get(words.len() - 1).unwrap());

    match (point1, point2) {
        (Some(p), Some(q)) => Some(Instruction {action, point1: p, point2: q}),
        _ => None
    }
}

fn get_rectangle(p: Point, q: Point) -> Option<Vec<Point>> {
    match (p, q) {
        (Point(x1, y1), Point(x2, y2)) 
        if x1 < 1000 && y1 < 1000 && x2 < 1000 && y2 < 1000
        && x1 <= x2 && y1 <= y2 => {
            Some(
                (x1..x2 + 1).flat_map(move |x| {
                    (y1..y2 + 1).map(move |y| {
                        Point(x, y)
                    })
                }).collect()
            )
        },
        _ => None
    }
}

fn main() {
    let input = get_input().unwrap();
    let instructions: Vec<Instruction> = input.lines().filter_map(parse_line).collect();

    let mut lit_grid = [[false; 1000]; 1000];
    let mut brightness_grid = (0..1000).map(|_| {
        (0..1000).map(|_| 0).collect::<Vec<_>>()
    }).collect::<Vec<_>>();

    for Instruction {action, point1, point2} in instructions.into_iter() {
        let rectangle = match get_rectangle(point1, point2) {
            Some(x) => x,
            None => continue
        };

        for Point(x, y) in rectangle.into_iter() {
            lit_grid[x][y] = match action {
                Action::Toggle => !lit_grid[x][y],
                Action::TurnOn => true,
                Action::TurnOff => false
            };

            brightness_grid[x][y] += match action {
                Action::Toggle => 2,
                Action::TurnOn => 1,
                Action::TurnOff if brightness_grid[x][y] > 0 => -1,
                _ => 0
            };
        }
    }

    let lit_count = lit_grid.into_iter()
        .flat_map(|x| x.into_iter())
        .filter(|&&x| x)
        .count();

    let brightness = brightness_grid.into_iter()
        .flat_map(|x| x.into_iter())
        .sum::<i32>();

    println!("Part 1: {}", lit_count);
    println!("Part 2: {}", brightness);
}
