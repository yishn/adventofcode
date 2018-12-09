use std::fs::File;
use std::io::prelude::*;
use std::collections::HashMap;
use std::hash::Hash;

fn get_input() -> std::io::Result<String> {
    let mut file = File::open("09.txt")?;
    let mut contents = String::new();
    file.read_to_string(&mut contents)?;

    Ok(contents)
}

struct CircularList<T: Eq + Copy + Hash> {
    len: usize,
    links: HashMap<T, (T, T)>
}

impl<T: Eq + Copy + Hash> CircularList<T> {
    fn new(init: T) -> CircularList<T> {
        CircularList {
            len: 1,
            links: {
                let mut links = HashMap::new();
                links.insert(init, (init, init));
                links
            }
        }
    }

    fn step(&self, value: T, step: isize) -> T {
        if step > 0 {
            let (_, next) = self.links.get(&value).cloned().unwrap();
            self.step(next, step - 1)
        } else if step < 0 {
            let (prev, _) = self.links.get(&value).cloned().unwrap();
            self.step(prev, step + 1)
        } else {
            value
        }
    }

    fn insert(&mut self, parent: T, value: T) {
        let parent_link = self.links.get(&parent).cloned().unwrap();
        let child_link = self.links.get(&parent_link.1).cloned().unwrap();

        self.len += 1;

        if self.len == 2 {
            let link = (parent, parent);

            self.links.insert(value, link);
            self.links.insert(parent, (value, value));
        } else {
            let link = (parent, parent_link.1);

            self.links.insert(value, link);
            self.links.insert(parent, (parent_link.0, value));
            self.links.insert(parent_link.1, (value, child_link.1));
        }
    }

    fn remove(&mut self, value: T) {
        if self.len == 0 {
            return;
        }

        let link = self.links.get(&value).cloned().unwrap();
        let parent_link = self.links.get(&link.0).cloned().unwrap();
        let child_link = self.links.get(&link.1).cloned().unwrap();

        self.len -= 1;

        if self.len == 1 {
            self.links.insert(link.0, (link.0, link.0));
        } else {
            self.links.insert(link.0, (parent_link.0, link.1));
            self.links.insert(link.1, (link.0, child_link.1));
        }
    }
}

fn play(players: usize, rounds: usize) -> HashMap<usize, usize> {
    let mut marbles = CircularList::new(0);
    let mut current = 0;
    let mut scores = HashMap::new();

    for i in 1..rounds + 1 {
        if i % 23 == 0 {
            let player = (i - 1) % players;
            let score = scores.get(&player).cloned().unwrap_or(0);
            let remove = marbles.step(current, -7);

            current = marbles.step(remove, 1);
            marbles.remove(remove);
            scores.insert(player, score + remove + i);
        } else {
            let parent = marbles.step(current, 1);

            marbles.insert(parent, i);
            current = i;
        }
    }

    scores
}

fn main() {
    let input = get_input().unwrap();
    let (players, rounds) = {
        let mut tokens = input.split(' ');

        match (tokens.next(), tokens.nth(5)) {
            (Some(x), Some(y)) => (x.parse::<usize>().ok().unwrap_or(0), y.parse::<usize>().ok().unwrap_or(0)),
            _ => (0, 0)
        }
    };

    let scores = play(players, rounds);
    println!("Part 1: {}", scores.values().cloned().max().unwrap_or(0));

    let scores = play(players, rounds * 100);
    println!("Part 2: {}", scores.values().cloned().max().unwrap_or(0));
}
