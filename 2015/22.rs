use std::cmp;
use std::collections::VecDeque;
use AttackResult::*;
use EffectClass::*;

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
struct Character {
    hp: i32,
    mana: i32,
    damage: i32,
    armor: i32
}

#[derive(Debug, Clone)]
enum AttackResult {
    Win,
    Lose,
    Continue(State)
}

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
enum EffectClass {
    Shield,
    Poison,
    Recharge
}

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
struct Effect {
    turns: i32,
    intensity: i32,
    class: EffectClass
}

#[derive(Debug, Copy, Clone)]
struct Spell {
    cost: i32,
    damage: i32,
    healing: i32,
    effect: Option<Effect>
}

#[derive(Debug, Clone, PartialEq, Eq)]
struct State {
    player: Character,
    boss: Character,
    effects: Vec<Effect>
}

fn process_effects(state: &mut State) {
    let mut effects = Vec::new();
    let mut armor = 0;

    for mut effect in state.effects.iter().filter(|x| x.turns > 0).cloned() {
        effect.turns -= 1;

        if effect.turns > 0 {
            effects.push(effect);
        }

        match effect.class {
            Shield => armor = effect.intensity,
            Poison => state.boss.hp -= effect.intensity,
            Recharge => state.player.mana += effect.intensity
        }
    }

    state.player.armor = armor;
    state.effects = effects;
}

fn cast_spell(state: &State, spell: Spell) -> AttackResult {
    let mut result = state.clone();
    process_effects(&mut result);

    // Cast spell

    result.player.mana -= spell.cost;
    result.player.hp += spell.healing;
    result.boss.hp -= spell.damage;

    if result.player.mana <= 0 {
        return Lose;
    }

    // Deal with effects

    if let Some(effect) = spell.effect {
        if result.effects.iter().all(|x| x.class != effect.class) {
            result.effects.push(effect);
        } else {
            return Lose;
        }
    }

    // Counter attack

    process_effects(&mut result);

    if result.boss.hp > 0 {
        let damage = cmp::max(result.boss.damage - result.player.armor, 1);
        result.player.hp = cmp::max(result.player.hp - damage, 0);

        if result.player.hp <= 0 {
            return Lose;
        }
    } else {
        return Win;
    }

    Continue(result)
}

fn cheapest_strategy(spells: &Vec<Spell>, start: &State) -> Option<Vec<Spell>> {
    let mut queue: VecDeque<(Vec<Spell>, State)> = VecDeque::new();
    let mut min = None;

    queue.push_back((vec![], start.clone()));

    while queue.len() > 0 {
        let (substrategy, state) = queue.pop_front().unwrap();
        let cost = substrategy.iter().map(|spell| spell.cost).sum::<i32>();

        for &spell in spells {
            match cast_spell(&state, spell) {
                Lose => continue,
                attack_result => {
                    if let Some((min_cost, _)) = min {
                        if cost + spell.cost >= min_cost {
                            continue;
                        }
                    }

                    let mut strategy = substrategy.clone();
                    strategy.push(spell);

                    if let Continue(next_state) = attack_result {
                        queue.push_back((strategy, next_state));
                    } else {
                        min = Some((cost + spell.cost, strategy));
                    }
                }
            }
        }
    }

    match min {
        None => None,
        Some((_, mut strategy)) => {
            strategy.reverse();
            Some(strategy)
        }
    }
}

fn main() {
    let spells = vec![
        Spell {
            cost: 53,
            damage: 4,
            healing: 0,
            effect: None
        },
        Spell {
            cost: 73,
            damage: 2,
            healing: 2,
            effect: None
        },
        Spell {
            cost: 113,
            damage: 0,
            healing: 0,
            effect: Some(Effect {turns: 6, intensity: 7, class: Shield})
        },
        Spell {
            cost: 173,
            damage: 0,
            healing: 0,
            effect: Some(Effect {turns: 6, intensity: 3, class: Poison})
        },
        Spell {
            cost: 229,
            damage: 0,
            healing: 0,
            effect: Some(Effect {turns: 5, intensity: 101, class: Recharge})
        }
    ];

    let boss = Character {
        hp: 51,
        mana: 0,
        damage: 9,
        armor: 0
    };

    let player = Character {
        hp: 50,
        mana: 500,
        damage: 0,
        armor: 0
    };

    let start = State {player, boss, effects: Vec::new()};
    let strategy = cheapest_strategy(&spells, &start).unwrap();
    let cost = strategy.iter().map(|spell| spell.cost).sum::<i32>();

    println!("Part 1: {}", cost);
}
