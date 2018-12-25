use std::fs::File;
use std::io::prelude::*;
use std::collections::HashMap;

fn get_input() -> std::io::Result<String> {
    let mut file = File::open("24.txt")?;
    let mut contents = String::new();
    file.read_to_string(&mut contents)?;

    Ok(contents)
}

#[derive(Debug, Clone)]
struct GroupInfo {
    id: usize,
    friendly: bool,
    units: usize,
    hp: usize,
    weaknesses: Vec<String>,
    immunities: Vec<String>,
    attack_type: String,
    damage: usize,
    initiative: usize
}

fn parse(input: &str) -> Vec<GroupInfo> {
    let input = input.trim().replace('\r', "");

    input.split("\n\n")
    .enumerate()
    .flat_map(|(i, part)| {
        part.trim().lines()
        .filter(|line| !line.contains(':'))
        .map(|line| line.split(|c| ['(', ')'].contains(&c)).collect::<Vec<_>>())
        .filter_map(|line_parts| match line_parts.len() {
            1 => Some((line_parts[0].trim().split(' ').chain("".split(' ')), None)),
            3 => Some((
                line_parts[0].trim().split(' ').chain(line_parts[2].trim().split(' ')),
                Some(line_parts[1])
            )),
            _ => None
        })
        .enumerate()
        .filter_map(move |(j, (mut part1, part2))| {
            let friendly = i == 0;
            let units = part1.nth(0)
                .and_then(|x| x.parse::<usize>().ok())
                .unwrap_or(0);
            let hp = part1.nth(3)
                .and_then(|x| x.parse::<usize>().ok())
                .unwrap_or(0);
            let damage = part1.nth(7)
                .and_then(|x| x.parse::<usize>().ok())
                .unwrap_or(0);
            let attack_type = part1.nth(0)
                .map(|x| x.to_string())
                .unwrap_or(String::new());
            let initiative = part1.nth(3)
                .and_then(|x| x.parse::<usize>().ok())
                .unwrap_or(0);

            let (weaknesses, immunities) = part2
                .map(|s| {
                    let mut hash_map = s.split("; ")
                        .filter_map(|part| {
                            let mut tokens = part.split(", ").collect::<Vec<_>>();
                            let key = tokens.get(0).and_then(|x| x.split(' ').next());

                            tokens.get_mut(0)
                            .and_then(|x| x.split(' ').nth(2).map(|v| *x = v));

                            key.map(|key| (key, tokens))
                        })
                        .fold(HashMap::new(), |mut acc, (k, v)| {
                            acc.insert(k, v);
                            acc
                        });

                    (
                        hash_map.remove(&"weak").unwrap_or_else(|| vec![]),
                        hash_map.remove(&"immune").unwrap_or_else(|| vec![])
                    )
                })
                .map(|(v, w)| (
                    v.into_iter().map(|x| x.to_string()).collect(),
                    w.into_iter().map(|x| x.to_string()).collect()
                ))
                .unwrap_or_else(|| (vec![], vec![]));

            Some(GroupInfo {
                id: i * 100 + j,
                friendly,
                units,
                hp,
                weaknesses,
                immunities,
                attack_type,
                damage,
                initiative
            })
        })
    })
    .collect()
}

fn select_target<'a>(group: &GroupInfo, targets: &[&GroupInfo]) -> Option<usize> {
    targets.iter()
    .map(|x| *x)
    .filter(|target| target.friendly != group.friendly)
    .filter(|target| get_actual_damage(group, target) > 0)
    .max_by_key(|target| (
        get_actual_damage(group, target),
        get_effective_power(target),
        target.initiative
    ))
    .map(|target| target.id)
}

fn get_attack_order(groups: &mut [GroupInfo]) -> Vec<(usize, Option<usize>)> {
    let mut result = Vec::new();
    let mut selected = Vec::new();

    groups.sort_by_key(|g| (get_effective_power(g), g.initiative));
    groups.reverse();

    for group in groups.iter() {
        let targets = groups.iter()
            .filter(|target| !selected.contains(&target.id))
            .collect::<Vec<_>>();

        let target = select_target(group, &targets);

        target.map(|t| selected.push(t));
        result.push((group.initiative, group.id, target));
    }

    result.sort();
    result.reverse();

    result.into_iter()
    .map(|(_, id, target)| (id, target))
    .collect()
}

fn get_effective_power(group: &GroupInfo) -> usize {
    group.units * group.damage
}

fn get_actual_damage(group: &GroupInfo, target: &GroupInfo) -> usize {
    let mut damage = get_effective_power(group);

    if target.immunities.contains(&group.attack_type) {
        damage = 0;
    } else if target.weaknesses.contains(&group.attack_type) {
        damage *= 2;
    }

    damage
}

fn attack(group: &GroupInfo, target: &GroupInfo) -> Option<GroupInfo> {
    let actual_damage = get_actual_damage(group, target);
    let lost_units = actual_damage / target.hp;

    if target.units > lost_units {
        let mut after = target.clone();
        after.units = target.units - lost_units;
        Some(after)
    } else {
        None
    }
}

fn fight(mut groups: Vec<GroupInfo>) -> Option<Vec<GroupInfo>> {
    while groups.len() > 0 && !groups.iter().all(|g| g.friendly == groups[0].friendly) {
        let attack_order = get_attack_order(&mut groups);
        let mut change = false;

        for (attacker_id, defender_id) in attack_order.into_iter() {
            if defender_id.is_none() {
                continue;
            }

            let defender_id = defender_id.unwrap();
            let mut defender_units = 0;

            let defender_after = {
                let attacker = groups.iter().find(|g| g.id == attacker_id);
                let defender = groups.iter().find(|g| g.id == defender_id);

                match (attacker, defender) {
                    (Some(x), Some(y)) => {
                        defender_units = y.units;
                        Some(attack(x, y))
                    },
                    _ => None
                }
            };

            groups = match defender_after {
                Some(Some(defender_after)) => {
                    // Defender lives

                    if defender_after.units != defender_units {
                        change = true;
                    }

                    groups.into_iter()
                    .map(|g| if g.id == defender_id { defender_after.clone() } else { g })
                    .collect()
                },
                Some(None) => {
                    // Defender dies

                    change = true;

                    groups.into_iter()
                    .filter(|g| g.id != defender_id)
                    .collect()
                },
                None => {
                    // Attacker or defender died already

                    groups
                }
            };
        }

        if !change {
            return None;
        }
    }

    Some(groups)
}

fn main() {
    let input = get_input().unwrap();
    let groups = parse(&input);

    fight(groups.clone())
    .map(|final_groups| {
        final_groups.iter()
        .map(|g| g.units)
        .sum::<usize>()
    })
    .map(|unit_count| println!("Part 1: {}", unit_count));

    (1..)
    .inspect(|x| if x % 1000 == 0 { println!("{}", x) })
    .filter_map(|boost| {
        let boosted_groups = groups.iter()
            .cloned()
            .map(|mut group| {
                if group.friendly {
                    group.damage += boost;
                }

                group
            })
            .collect::<Vec<_>>();

        fight(boosted_groups)
    })
    .find(|final_groups| final_groups.len() > 0 && final_groups[0].friendly)
    .map(|final_groups| {
        final_groups.iter()
        .map(|g| g.units)
        .sum::<usize>()
    })
    .map(|unit_count| println!("Part 2: {}", unit_count));
}
