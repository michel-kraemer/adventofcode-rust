use std::{cmp::max, collections::HashMap, fs};

fn play(
    player_points: i32,
    player_damage: i32,
    player_armor: i32,
    boss_points: i32,
    boss_damage: i32,
    boss_armor: i32,
) -> bool {
    let mut player_points = player_points;
    let mut boss_points = boss_points;

    while player_points > 0 && boss_points > 0 {
        let pd = max(1, player_damage - boss_armor);
        boss_points -= pd;
        if boss_points <= 0 {
            break;
        }

        let bd = max(1, boss_damage - player_armor);
        player_points -= bd;
        if player_points <= 0 {
            break;
        }
    }

    player_points > 0
}

fn main() {
    let shop_input = fs::read_to_string("shop.txt").expect("Could not read file");

    let shop_categories = shop_input.split("\n\n").collect::<Vec<_>>();
    let shop_categories = shop_categories
        .into_iter()
        .map(|c| {
            c.lines()
                .skip(1)
                .map(|l| {
                    let values = l
                        .split(" ")
                        .filter(|i| !i.is_empty())
                        .skip(1)
                        .collect::<Vec<_>>();
                    (
                        values[values.len() - 3].parse::<i32>().unwrap(),
                        values[values.len() - 2].parse::<i32>().unwrap(),
                        values[values.len() - 1].parse::<i32>().unwrap(),
                    )
                })
                .collect::<Vec<_>>()
        })
        .collect::<Vec<_>>();

    let input = fs::read_to_string("input.txt").expect("Could not read file");
    let boss_stats = input
        .lines()
        .map(|l| l.split_once(": ").unwrap())
        .collect::<HashMap<_, _>>();

    let boss_points = boss_stats["Hit Points"].parse::<i32>().unwrap();
    let boss_damage = boss_stats["Damage"].parse::<i32>().unwrap();
    let boss_armor = boss_stats["Armor"].parse::<i32>().unwrap();

    let player_points = 100;

    let weapons = &shop_categories[0];
    let armor = &shop_categories[1];
    let rings = &shop_categories[2];

    let mut won_games = Vec::new();
    let mut lost_games = Vec::new();

    for weapon in weapons {
        let mut c = weapon.0;
        let mut d = weapon.1;
        let mut a = weapon.2;

        for armor_i in 0..=armor.len() {
            if armor_i < armor.len() {
                c += armor[armor_i].0;
                d += armor[armor_i].1;
                a += armor[armor_i].2;
            }

            for ring_i1 in 0..=rings.len() {
                if ring_i1 < rings.len() {
                    c += rings[ring_i1].0;
                    d += rings[ring_i1].1;
                    a += rings[ring_i1].2;
                }

                for ring_i2 in 0..=rings.len() {
                    if ring_i2 != rings.len() && ring_i2 == ring_i1 {
                        continue;
                    }

                    if ring_i2 < rings.len() {
                        c += rings[ring_i2].0;
                        d += rings[ring_i2].1;
                        a += rings[ring_i2].2;
                    }

                    if play(player_points, d, a, boss_points, boss_damage, boss_armor) {
                        won_games.push(c);
                    } else {
                        lost_games.push(c);
                    }

                    if ring_i2 < rings.len() {
                        c -= rings[ring_i2].0;
                        d -= rings[ring_i2].1;
                        a -= rings[ring_i2].2;
                    }
                }

                if ring_i1 < rings.len() {
                    c -= rings[ring_i1].0;
                    d -= rings[ring_i1].1;
                    a -= rings[ring_i1].2;
                }
            }

            if armor_i < armor.len() {
                c -= armor[armor_i].0;
                d -= armor[armor_i].1;
                a -= armor[armor_i].2;
            }
        }
    }

    println!("{}", won_games.into_iter().min().unwrap());
    println!("{}", lost_games.into_iter().max().unwrap());
}
