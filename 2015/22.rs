use std::cmp;
use AttackResult::*;
use EffectClass::*;

#[derive(Debug, Copy, Clone)]
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

#[derive(Debug, Copy, Clone, PartialEq)]
enum EffectClass {
    Shield,
    Poison,
    Recharge
}

#[derive(Debug, Copy, Clone)]
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

#[derive(Debug, Clone)]
struct State {
    player: Character,
    boss: Character,
    effects: Vec<Effect>
}

fn process_effects(state: &State) -> State {
    let mut result = State {player: state.player, boss: state.boss, effects: Vec::new()};
    let mut armor = 0;

    for mut effect in state.effects.iter().filter(|x| x.turns > 0).cloned() {
        effect.turns -= 1;

        if effect.turns > 0 {
            result.effects.push(effect);
        }

        match effect.class {
            Shield => armor = effect.intensity,
            Poison => result.boss.hp -= effect.intensity,
            Recharge => result.player.mana += effect.intensity
        }
    }

    result.player.armor = armor;
    result
}

fn cast_spell(state: &State, spell: Spell) -> Option<AttackResult> {
    let mut result = process_effects(state);

    // Cast spell

    result.player.mana -= spell.cost;
    result.player.hp += spell.healing;
    result.boss.hp -= spell.damage;

    if result.player.mana <= 0 {
        return Some(Lose);
    }

    // Deal with effects

    if let Some(effect) = spell.effect {
        if result.effects.iter().all(|x| x.class != effect.class) {
            result.effects.push(effect)
        } else {
            return None;
        }
    }

    // Counter attack

    let mut result = process_effects(&result);

    if result.boss.hp > 0 {
        let damage = cmp::max(result.boss.damage - result.player.armor, 1);
        result.player.hp = cmp::max(result.player.hp - damage, 0);

        if result.player.hp <= 0 {
            return Some(Lose);
        }
    } else {
        return Some(Win);
    }

    Some(Continue(result))
}

fn list_strategies(spells: &Vec<Spell>, state: &State) -> Vec<Vec<Spell>> {
    spells.iter().flat_map(|&spell| match cast_spell(state, spell) {
        Some(Continue(next_state)) => {
            list_strategies(spells, &next_state)
            .into_iter()
            .map(|mut strategy| {
                strategy.push(spell);
                strategy
            })
            .collect()
        },
        Some(Win) => vec![vec![spell]],
        _ => vec![]
    }).collect()
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
    let strategies = list_strategies(&spells, &start);

    println!("{:?}", strategies.len());
}
