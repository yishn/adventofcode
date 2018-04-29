use std::fs::File;
use std::io::Read;

fn get_input() -> std::io::Result<String> {
    let mut file = File::open("14.txt")?;
    let mut contents = String::new();
    file.read_to_string(&mut contents)?;

    Ok(contents)
}

struct Reindeer {
    speed: i32,
    action: i32,
    rest: i32
}

impl Reindeer {
    fn distance(&self, t: i32) -> i32 {
        if t <= 0 {
            return 0;
        }

        let period = self.action + self.rest;
        let period_number = (t / period) as i32;
        let period_distance = self.action * self.speed;
        let rest_time = t - period_number * period;

        period_number * period_distance + match rest_time {
            rt if rt <= self.action => rt * self.speed,
            _ => self.action * self.speed
        }
    }
}

fn parse_line(line: &str) -> Option<Reindeer> {
    let tokens = line.split(' ').collect::<Vec<_>>();
    if tokens.len() < 14 {
        return None;
    }

    let speed = match tokens[3].parse::<i32>() {
        Ok(x) => x,
        Err(_) => return None
    };

    let action = match tokens[6].parse::<i32>() {
        Ok(x) => x,
        Err(_) => return None
    };

    let rest = match tokens[13].parse::<i32>() {
        Ok(x) => x,
        Err(_) => return None
    };

    Some(Reindeer {speed, action, rest})
}

fn main() {
    let input = get_input().unwrap();
    let reindeers = input.lines().filter_map(parse_line).collect::<Vec<_>>();
    let end = 2503;
    let max_distance = reindeers.iter().map(|r| r.distance(end)).max().unwrap();

    println!("Part 1: {}", max_distance);

    let mut points = reindeers.iter().map(|_| 0).collect::<Vec<_>>();

    for t in 1..end + 1 {
        let distances = reindeers.iter().map(|r| r.distance(t)).collect::<Vec<_>>();
        let max_distance = distances.iter().max().unwrap();

        for (i, distance) in distances.iter().enumerate() {
            points[i] += match distance == max_distance {
                true => 1,
                false => 0
            };
        }
    }

    println!("Part 2: {}", points.iter().max().unwrap());
}
