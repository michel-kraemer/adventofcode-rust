use std::{
    cmp::{Reverse, max},
    collections::{BTreeMap, BinaryHeap, HashMap, HashSet},
    fs,
    hash::Hash,
};

#[derive(Debug, PartialEq, Eq, Hash, Clone, PartialOrd, Ord)]
enum Spell {
    MagicMissile,
    Drain,
    Shield,
    Poison,
    Recharge,
}

#[derive(Debug, PartialEq, Eq, Clone, Hash)]
struct GameState {
    player_points: i32,
    player_mana: i32,
    player_armor: i32,
    boss_points: i32,
    boss_damage: i32,
    mana_spent: i32,
    effects: BTreeMap<Spell, i32>,
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

fn evaluate_effects(s: GameState) -> GameState {
    let mut ns = s.clone();
    for e in &s.effects {
        match *e.0 {
            Spell::Shield => {
                let e = ns.effects.get_mut(&Spell::Shield).unwrap();
                *e -= 1;
                if *e == 0 {
                    ns.effects.remove(&Spell::Shield);
                    ns.player_armor = 0;
                } else {
                    ns.player_armor = 7;
                }
            }
            Spell::Poison => {
                let e = ns.effects.get_mut(&Spell::Poison).unwrap();
                ns.boss_points -= 3;
                *e -= 1;
                if *e == 0 {
                    ns.effects.remove(&Spell::Poison);
                }
            }
            Spell::Recharge => {
                let e = ns.effects.get_mut(&Spell::Recharge).unwrap();
                ns.player_mana += 101;
                *e -= 1;
                if *e == 0 {
                    ns.effects.remove(&Spell::Recharge);
                }
            }
            _ => unreachable!(),
        };
    }
    ns
}

fn main() {
    for part1 in [true, false] {
        let input = fs::read_to_string("input.txt").expect("Could not read file");
        let boss_stats = input
            .lines()
            .map(|l| l.split_once(": ").unwrap())
            .collect::<HashMap<_, _>>();

        let boss_points = boss_stats["Hit Points"].parse::<i32>().unwrap();
        let boss_damage = boss_stats["Damage"].parse::<i32>().unwrap();

        let player_points = 50;
        let player_mana = 500;

        let mut queue = BinaryHeap::new();
        queue.push(Reverse(GameState {
            player_points,
            player_mana,
            player_armor: 0,
            boss_points,
            boss_damage,
            mana_spent: 0,
            effects: BTreeMap::new(),
        }));

        let mut seen = HashSet::new();
        let mut result = 0;

        'outer: while let Some(Reverse(s)) = queue.pop() {
            for spell in [
                Spell::MagicMissile,
                Spell::Drain,
                Spell::Shield,
                Spell::Poison,
                Spell::Recharge,
            ] {
                let immediate_damage = if part1 { 0 } else { 1 };

                let mut s = s.clone();
                s.player_points -= immediate_damage;
                if s.player_points <= 0 {
                    continue;
                }

                let mut s = evaluate_effects(s);
                if s.boss_points <= 0 {
                    result = s.mana_spent;
                    break 'outer;
                }

                if s.effects.contains_key(&spell) {
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
                        s.effects.insert(Spell::Shield, 6);
                    }
                    Spell::Poison => {
                        if s.player_mana <= 173 {
                            continue;
                        }
                        s.player_mana -= 173;
                        s.mana_spent += 173;
                        s.effects.insert(Spell::Poison, 6);
                    }
                    Spell::Recharge => {
                        if s.player_mana <= 229 {
                            continue;
                        }
                        s.player_mana -= 229;
                        s.mana_spent += 229;
                        s.effects.insert(Spell::Recharge, 5);
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

                s.player_points -= max(1, s.boss_damage - s.player_armor);
                if s.player_points <= 0 {
                    continue;
                }

                if !seen.contains(&s) {
                    seen.insert(s.clone());
                    queue.push(Reverse(s));
                }
            }
        }

        println!("{result}");
    }
}
