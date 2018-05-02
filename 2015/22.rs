use EffectClass::*;

#[derive(Debug, Copy, Clone)]
struct Character {
    hp: i32,
    mana: i32,
    damage: i32
}

impl Character {
    fn new(hp: i32) -> Character {
        Character {hp, mana: 0, damage: 0}
    }
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
        damage: 9
    };

    let player = Character {
        hp: 50,
        mana: 500,
        damage: 0
    };
}
