use std::fmt;
use std::fs::File;
use std::io::prelude::*;
use std::collections::HashMap;

fn get_input() -> std::io::Result<String> {
    let mut file = File::open("04.txt")?;
    let mut contents = String::new();
    file.read_to_string(&mut contents)?;

    Ok(contents)
}

#[derive(Eq, PartialEq, Ord, PartialOrd, Copy, Clone)]
struct DateTime {
    year: usize,
    month: usize,
    day: usize,
    hour: usize,
    minute: usize
}

impl fmt::Debug for DateTime {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}-{}-{} {}:{}", self.year, self.month, self.day, self.hour, self.minute)
    }
}

#[derive(Debug, Eq, PartialEq, Ord, PartialOrd, Copy, Clone)]
enum Event {
    Begin(usize),
    Sleep,
    WakeUp
}

#[derive(Debug, Eq, PartialEq, Ord, PartialOrd)]
struct LogEntry {
    time: DateTime,
    event: Event
}

type AsleepPlan = HashMap<usize, HashMap<usize, Vec<DateTime>>>;

fn create_log(input: &str) -> Vec<LogEntry> {
    let mut result: Vec<LogEntry> = input.lines()
        .filter_map(|line| {
            let mut tokens = line.split(' ');

            let mut date = match tokens.next() {
                Some(x) => x[1..].split('-').filter_map(|x| x.parse::<usize>().ok()),
                _ => return None
            };

            let mut time = match tokens.next() {
                Some(x) => x[..x.len() - 1].split(':').filter_map(|x| x.parse::<usize>().ok()),
                _ => return None
            };

            let event = match (tokens.next(), tokens.next()) {
                (Some("wakes"), _) => Event::WakeUp,
                (Some("falls"), _) => Event::Sleep,
                (Some("Guard"), Some(token)) => match token[1..].parse::<usize>() {
                    Ok(id) => Event::Begin(id),
                    _ => return None
                },
                _ => return None
            };

            match (date.next(), date.next(), date.next(), time.next(), time.next()) {
                (Some(year), Some(month), Some(day), Some(hour), Some(minute)) => {
                    Some(LogEntry {
                        time: DateTime {year, month, day, hour, minute},
                        event
                    })
                },
                _ => None
            }
        })
        .collect();

    result.sort();
    result
}

fn create_plan(log: &Vec<LogEntry>) -> AsleepPlan {
    let mut plan = AsleepPlan::new();

    if log.len() == 0 {
        return plan;
    }

    let mut id = 0;
    let mut last_time = log[0].time;
    let mut asleep = true;

    for &LogEntry {time, event} in log.iter() {
        match event {
            Event::Begin(x) => {
                id = x;

                if !plan.contains_key(&id) {
                    plan.insert(id, HashMap::new());
                }
            },
            Event::WakeUp if asleep => {
                if let Some(asleep_map) = plan.get_mut(&id) {
                    for i in last_time.minute..time.minute {
                        if !asleep_map.contains_key(&i) {
                            asleep_map.insert(i, vec![]);
                        }

                        let asleep_vec = asleep_map.get_mut(&i).unwrap();

                        asleep_vec.push(DateTime {
                            year: time.year,
                            month: time.month,
                            day: time.day,
                            hour: time.hour,
                            minute: i
                        });
                    }
                }

                asleep = false;
            },
            Event::Sleep => {
                last_time = time;
                asleep = true;
            },
            _ => {}
        };
    }

    plan
}

fn main() {
    let data = create_log(&get_input().unwrap());
    let plan = create_plan(&data);

    let (candidate_id, candidate_minute) = plan.iter()
        .max_by_key(|&(_, asleep_map)| asleep_map.values().flatten().count())
        .map(|(&id, asleep_map)| (
            id,
            asleep_map.iter()
                .max_by_key(|&(_, asleep_vec)| asleep_vec.len())
                .map(|(&minute, _)| minute)
                .unwrap_or(0)
        ))
        .unwrap_or((0, 0));

    println!("Part 1: {}", candidate_id * candidate_minute);

    let (candidate_id, candidate_minute) = plan.iter()
        .map(|(&id, asleep_map)| (
            id,
            asleep_map.iter()
                .map(|(&minute, asleep_vec)| (minute, asleep_vec.len()))
                .max_by_key(|&(_, value)| value)
                .unwrap_or((0, 0))
        ))
        .max_by_key(|&(_, (_, value))| value)
        .map(|(x, (y, _))| (x, y))
        .unwrap_or((0, 0));

    println!("Part 2: {}", candidate_id * candidate_minute);
}
