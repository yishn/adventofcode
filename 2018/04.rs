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

#[derive(Eq, PartialEq, Ord, PartialOrd, Hash, Clone, Copy)]
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

#[derive(Debug, Eq, PartialEq, Ord, PartialOrd, Clone, Copy)]
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

type AsleepPlan = HashMap<usize, Vec<DateTime>>;

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
                    plan.insert(id, vec![]);
                }
            },
            Event::WakeUp if asleep => {
                if let Some(asleep_vec) = plan.get_mut(&id) {
                    for i in last_time.minute..time.minute {
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

    let vec = vec![];
    let (candidate_id, asleep_vec) = plan.iter()
        .fold(None, |acc, (&id, asleep_vec)| match acc {
            None => Some((id, asleep_vec)),
            Some((_, x)) => if x.len() > asleep_vec.len() {
                acc
            } else {
                Some((id, asleep_vec))
            }
        })
        .unwrap_or((0, &vec));

    let candidate_minute = asleep_vec.iter()
        .fold(HashMap::new(), |mut acc, &time| {
            let value = match acc.get(&time.minute) {
                Some(&x) => x,
                _ => 0
            };

            acc.insert(time.minute, value + 1);
            acc
        })
        .iter()
        .fold(None, |acc, (&minute, &n)| match acc {
            None => Some((minute, n)),
            Some((_, m)) => if m > n {
                acc
            } else {
                Some((minute, n))
            }
        })
        .unwrap_or((0, 0)).0;

    println!("Part 1: {}", candidate_id * candidate_minute);
}
