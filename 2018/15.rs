use std::fs::File;
use std::io::prelude::*;
use std::collections::{VecDeque, HashMap, HashSet};

fn get_input() -> std::io::Result<String> {
    let mut file = File::open("15.txt")?;
    let mut contents = String::new();
    file.read_to_string(&mut contents)?;

    Ok(contents)
}

#[derive(Debug, Eq, PartialEq, Copy, Clone)]
enum Terrain {
    Wall,
    Free
}

#[derive(Debug, Eq, PartialEq, Copy, Clone, Hash)]
enum CharacterClass {
    Goblin,
    Elf
}

#[derive(Debug, Clone)]
struct Character {
    position: Point,
    class: CharacterClass,
    hp: usize,
    attack: usize
}

impl Character {
    fn attack(&self, opponent_hp: usize) -> usize {
        if opponent_hp > self.attack {
            opponent_hp - self.attack
        } else {
            0
        }
    }
}

type Point = (isize, isize);
type GameMap = HashMap<Point, Terrain>;

fn parse_map(input: &str) -> (GameMap, Vec<Character>) {
    let mut characters = Vec::new();
    let map = input.lines()
        .enumerate()
        .flat_map(|(y, line)| {
            line.chars().enumerate().map(move |(x, c)| ((x as isize, y as isize), c))
        })
        .map(|((x, y), c)| (
            (x, y),
            match c {
                '#' => Terrain::Wall,
                c if c == 'G' || c == 'E' => {
                    characters.push(Character {
                        position: (x, y),
                        class: match c {
                            'G' => CharacterClass::Goblin,
                            _ => CharacterClass::Elf
                        },
                        hp: 200,
                        attack: 3
                    });

                    Terrain::Free
                },
                _ => Terrain::Free
            }
        ))
        .fold(GameMap::new(), |mut acc, (k, v)| {
            acc.insert(k, v);
            acc
        });

    (map, characters)
}

fn get_neighbors((x, y): Point, map: &GameMap, blocked: &[Point]) -> Vec<Point> {
    [(x, y - 1), (x - 1, y), (x + 1, y), (x, y + 1)]
    .into_iter()
    .filter(|p| map.get(p) == Some(&Terrain::Free) && !blocked.contains(p))
    .cloned()
    .collect()
}

fn get_all_neighbors(points: &[Point], map: &GameMap, blocked: &[Point]) -> Vec<Point> {
    points.iter()
    .flat_map(|&p| get_neighbors(p, map, blocked))
    .collect()
}

fn get_distances(p: Point, map: &GameMap, blocked: &[Point]) -> HashMap<Point, usize> {
    let mut queue = VecDeque::new();
    let mut distances = HashMap::new();

    queue.push_back((p, 0));

    while let Some((q, d)) = queue.pop_front() {
        if distances.contains_key(&q) {
            continue;
        }

        distances.insert(q, d);

        get_neighbors(q, map, blocked).into_iter()
        .fold((), |_, neighbor| queue.push_back((neighbor, d + 1)));
    }

    distances
}

fn get_target(c: &Character, map: &GameMap, characters: &[Character]) -> Option<Point> {
    let opponent_positions = characters.iter()
        .filter(|opp| opp.class != c.class)
        .map(|opp| opp.position)
        .collect::<Vec<_>>();

    if opponent_positions.len() == 0 {
        return None;
    }

    let blocked = characters.iter()
        .map(|c| c.position)
        .filter(|&p| p != c.position)
        .collect::<Vec<_>>();

    let area = get_distances(c.position, map, &blocked);
    let opponent_neighbors = get_all_neighbors(&opponent_positions, map, &blocked);

    area.keys()
    // In range
    .filter(|p| opponent_neighbors.contains(p))
    // Nearest
    .map(|&p| (p, area.get(&p).cloned().unwrap()))
    .min_by_key(|&((x, y), d)| (d, y, x))
    // Distance
    .map(|(p, _)| get_distances(p, map, &blocked))
    // Step
    .and_then(|target_distances| {
        get_neighbors(c.position, map, &blocked).into_iter()
        .chain([c.position].iter().cloned())
        .min_by_key(|p| (target_distances.get(p).cloned().unwrap(), p.1, p.0))
    })
}

fn play_round(map: &GameMap, characters: &mut Vec<Character>) -> (bool, bool) {
    characters.sort_by_key(|c| (c.position.1, c.position.0));

    let mut i = 0;
    let mut finished = true;
    let mut elf_died = false;

    while i < characters.len() {
        get_target(&characters[i], map, characters)
        .map(|next| {
            characters[i].position = next;

            let neighbors = get_neighbors(next, map, &[]);
            let opponent_index = characters.iter()
                .enumerate()
                .filter(|&(_, c)| c.class != characters[i].class && neighbors.contains(&c.position))
                .min_by_key(|&(_, c)| (c.hp, c.position.1, c.position.0))
                .map(|(j, _)| j);

            opponent_index.map(|j| {
                let new_hp = characters[i].attack(characters[j].hp);

                if new_hp == 0 {
                    let opponent_class = characters[j].class;

                    characters.remove(j);

                    if j < i {
                        i -= 1;
                    }

                    if opponent_class == CharacterClass::Elf {
                        elf_died = true;
                    }

                    if i + 1 < characters.len()
                    && characters.iter().all(|c| c.class != opponent_class) {
                        finished = false;
                    }
                } else {
                    characters[j].hp = new_hp;
                }
            });
        });

        i += 1;
    }

    (finished, elf_died)
}

fn play(map: &GameMap, characters: &mut Vec<Character>, abort_on_elf_death: bool) -> (usize, bool) {
    let mut i = 0;

    loop {
        let (finished, elf_died) = play_round(map, characters);
        let classes = characters.iter()
            .map(|c| c.class)
            .collect::<HashSet<_>>()
            .iter()
            .count();

        if finished {
            i += 1;
        }

        if abort_on_elf_death && elf_died {
            return (i, true)
        }

        if classes <= 1 {
            break;
        }
    }

    (i, false)
}

fn main() {
    let input = get_input().unwrap();
    let (map, original_characters) = parse_map(&input);

    let mut characters = original_characters.clone();
    let (finished_rounds, _) = play(&map, &mut characters, false);
    let hp_sum = characters.iter().map(|c| c.hp).sum::<usize>();

    println!("Part 1: {}", hp_sum * finished_rounds);

    for attack_boost in 1.. {
        let mut characters = original_characters.clone();

        characters.iter_mut()
        .filter(|c| c.class == CharacterClass::Elf)
        .fold((), |_, elf| {
            elf.attack += attack_boost;
        });

        let (finished_rounds, elf_died) = play(&map, &mut characters, true);

        if !elf_died {
            let hp_sum = characters.iter().map(|c| c.hp).sum::<usize>();

            println!("Part 2: {}", hp_sum * finished_rounds);
            break;
        }
    }
}
