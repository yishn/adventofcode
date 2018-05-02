use std::cmp;
use ItemCategory::*;

struct Character {
    hp: i32,
    damage: i32,
    armor: i32
}

impl Character {
    fn new(hp: i32) -> Character {
        Character {hp, damage: 0, armor: 0}
    }
}

#[derive(Debug, PartialEq)]
enum ItemCategory {
    Weapon,
    Armor,
    Ring
}

#[derive(Debug)]
struct Item {
    category: ItemCategory,
    cost: i32,
    damage: i32,
    armor: i32
}

fn get_damage(attacker: &Character, defender: &Character) -> i32 {
    cmp::max(attacker.damage - defender.armor, 1)
}

fn is_winner<'a>(player: &'a Character, target: &'a Character) -> bool {
    let player_damage = get_damage(player, target);
    let target_damage = get_damage(target, player);
    let player_turns = (player.hp as f32 / target_damage as f32).ceil();
    let target_turns = (target.hp as f32 / player_damage as f32).ceil();

    target_turns <= player_turns
}

fn list_item_combinations<'a>(items: &'a Vec<&'a Item>) -> Vec<Vec<&'a Item>> {
    let weapons = items.iter().cloned().filter(|x| x.category == Weapon).collect::<Vec<_>>();
    let armors = items.iter().cloned().filter(|x| x.category == Armor).collect::<Vec<_>>();
    let rings = items.iter().cloned().filter(|x| x.category == Ring).collect::<Vec<_>>();
    let mut choices = Vec::new();

    for &weapon in weapons.iter() {
        choices.push(vec![weapon]);

        for i in 0..rings.len() {
            choices.push(vec![weapon, rings[i]]);
            for j in i + 1..rings.len() {
                choices.push(vec![weapon, rings[i], rings[j]]);
            }
        }

        for &armor in armors.iter() {
            choices.push(vec![weapon, armor]);

            for i in 0..rings.len() {
                choices.push(vec![weapon, armor, rings[i]]);
                for j in i + 1..rings.len() {
                    choices.push(vec![weapon, armor, rings[i], rings[j]]);
                }
            }
        }
    }

    choices
}

fn get_character(hp: i32, items: &Vec<&Item>) -> Character {
    items.iter().fold(Character::new(hp), |mut character, item| {
        character.damage += item.damage;
        character.armor += item.armor;
        character
    })
}

fn get_cheapest_winning_items<'a>(
    items: &'a Vec<&'a Item>,
    hp: i32,
    target: &Character
) -> Option<Vec<&'a Item>> {
    list_item_combinations(items).into_iter()
    .filter(|combination| {
        is_winner(&get_character(hp, combination), target)
    })
    .min_by_key(|combination| {
        combination.iter()
        .map(|item| item.cost)
        .sum::<i32>()
    })
}

fn get_costliest_losing_items<'a>(
    items: &'a Vec<&'a Item>,
    hp: i32,
    target: &Character
) -> Option<Vec<&'a Item>> {
    list_item_combinations(items).into_iter()
    .filter(|combination| {
        !is_winner(&get_character(hp, combination), target)
    })
    .max_by_key(|combination| {
        combination.iter()
        .map(|item| item.cost)
        .sum::<i32>()
    })
}

fn main() {
    let items = vec![
        Item {
            category: Weapon,
            cost: 8,
            damage: 4,
            armor: 0
        },
        Item {
            category: Weapon,
            cost: 10,
            damage: 5,
            armor: 0
        },
        Item {
            category: Weapon,
            cost: 25,
            damage: 6,
            armor: 0
        },
        Item {
            category: Weapon,
            cost: 40,
            damage: 7,
            armor: 0
        },
        Item {
            category: Weapon,
            cost: 74,
            damage: 8,
            armor: 0
        },
        Item {
            category: Armor,
            cost: 13,
            damage: 0,
            armor: 1
        },
        Item {
            category: Armor,
            cost: 31,
            damage: 0,
            armor: 2
        },
        Item {
            category: Armor,
            cost: 53,
            damage: 0,
            armor: 3
        },
        Item {
            category: Armor,
            cost: 75,
            damage: 0,
            armor: 4
        },
        Item {
            category: Armor,
            cost: 102,
            damage: 0,
            armor: 5
        },
        Item {
            category: Ring,
            cost: 25,
            damage: 1,
            armor: 0
        },
        Item {
            category: Ring,
            cost: 50,
            damage: 2,
            armor: 0
        },
        Item {
            category: Ring,
            cost: 100,
            damage: 3,
            armor: 0
        },
        Item {
            category: Ring,
            cost: 20,
            damage: 0,
            armor: 1
        },
        Item {
            category: Ring,
            cost: 40,
            damage: 0,
            armor: 2
        },
        Item {
            category: Ring,
            cost: 80,
            damage: 0,
            armor: 3
        }
    ];

    let boss = Character {
        hp: 100,
        damage: 8,
        armor: 2
    };

    let items = items.iter().collect();

    let combinations = get_cheapest_winning_items(&items, 100, &boss).unwrap();
    println!("Part 1: {}", combinations.into_iter().map(|item| item.cost).sum::<i32>());

    let combinations = get_costliest_losing_items(&items, 100, &boss).unwrap();
    println!("Part 2: {}", combinations.into_iter().map(|item| item.cost).sum::<i32>());
}
