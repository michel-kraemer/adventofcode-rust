use std::{cmp::Reverse, collections::BinaryHeap, fs, hash::Hash};

use rustc_hash::FxHashSet;

#[derive(Debug, PartialEq, Eq, Hash, Clone, PartialOrd, Ord)]
enum Spell {
    MagicMissile,
    Drain,
    Shield,
    Poison,
    Recharge,
}

#[derive(Debug, Default, PartialEq, Eq, Clone, Copy, Hash)]
struct Effects {
    shield: i32,
    poison: i32,
    recharge: i32,
}

#[derive(Debug, PartialEq, Eq, Clone, Copy, Hash)]
struct GameState {
    player_points: i32,
    player_mana: i32,
    player_armor: i32,
    boss_points: i32,
    boss_damage: i32,
    mana_spent: i32,
    effects: Effects,
}

impl Ord for GameState {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.mana_spent.cmp(&other.mana_spent)
    }
}

impl PartialOrd for GameState {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

fn evaluate_effects(mut s: GameState) -> GameState {
    if s.effects.shield > 0 {
        s.effects.shield -= 1;
        if s.effects.shield == 0 {
            s.player_armor = 0;
        } else {
            s.player_armor = 7;
        }
    }
    if s.effects.poison > 0 {
        s.boss_points -= 3;
        s.effects.poison -= 1;
    }
    if s.effects.recharge > 0 {
        s.player_mana += 101;
        s.effects.recharge -= 1;
    }
    s
}

fn main() {
    let input = fs::read_to_string("input.txt").expect("Could not read file");
    let mut boss_stats = input.lines().map(|l| {
        let p = l.split_once(": ").unwrap();
        p.1.parse::<i32>().unwrap()
    });

    let boss_points = boss_stats.next().unwrap();
    let boss_damage = boss_stats.next().unwrap();

    let player_points = 50;
    let player_mana = 500;

    for part1 in [true, false] {
        let mut queue = BinaryHeap::new();
        queue.push(Reverse(GameState {
            player_points,
            player_mana,
            player_armor: 0,
            boss_points,
            boss_damage,
            mana_spent: 0,
            effects: Effects::default(),
        }));

        let mut seen = FxHashSet::default();
        let mut result = 0;

        'outer: while let Some(Reverse(s)) = queue.pop() {
            for spell in [
                Spell::MagicMissile,
                Spell::Drain,
                Spell::Shield,
                Spell::Poison,
                Spell::Recharge,
            ] {
                let mut s = s;
                if !part1 {
                    s.player_points -= 1;
                    if s.player_points <= 0 {
                        continue;
                    }
                }

                let mut s = evaluate_effects(s);
                if s.boss_points <= 0 {
                    result = s.mana_spent;
                    break 'outer;
                }

                if (spell == Spell::Shield && s.effects.shield > 0)
                    || (spell == Spell::Poison && s.effects.poison > 0)
                    || (spell == Spell::Recharge && s.effects.recharge > 0)
                {
                    continue;
                }

                match spell {
                    Spell::MagicMissile => {
                        if s.player_mana <= 53 {
                            continue;
                        }
                        s.player_mana -= 53;
                        s.boss_points -= 4;
                        s.mana_spent += 53;
                    }
                    Spell::Drain => {
                        if s.player_mana <= 73 {
                            continue;
                        }
                        s.player_points += 2;
                        s.player_mana -= 73;
                        s.boss_points -= 2;
                        s.mana_spent += 73;
                    }
                    Spell::Shield => {
                        if s.player_mana <= 113 {
                            continue;
                        }
                        s.player_mana -= 113;
                        s.player_armor = 7;
                        s.mana_spent += 113;
                        s.effects.shield = 6;
                    }
                    Spell::Poison => {
                        if s.player_mana <= 173 {
                            continue;
                        }
                        s.player_mana -= 173;
                        s.mana_spent += 173;
                        s.effects.poison = 6;
                    }
                    Spell::Recharge => {
                        if s.player_mana <= 229 {
                            continue;
                        }
                        s.player_mana -= 229;
                        s.mana_spent += 229;
                        s.effects.recharge = 5;
                    }
                };

                if s.boss_points <= 0 {
                    result = s.mana_spent;
                    break 'outer;
                }

                let mut s = evaluate_effects(s);
                if s.boss_points <= 0 {
                    result = s.mana_spent;
                    break 'outer;
                }

                s.player_points -= (s.boss_damage - s.player_armor).max(1);
                if s.player_points <= 0 {
                    continue;
                }

                if !seen.contains(&s) {
                    seen.insert(s);
                    queue.push(Reverse(s));
                }
            }
        }

        println!("{result}");
    }
}
