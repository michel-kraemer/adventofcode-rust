use std::fs;

const WEAPONS: [(i32, i32, i32); 5] = [(8, 4, 0), (10, 5, 0), (25, 6, 0), (40, 7, 0), (74, 8, 0)];

const ARMOR: [(i32, i32, i32); 5] = [(13, 0, 1), (31, 0, 2), (53, 0, 3), (75, 0, 4), (102, 0, 5)];

const RINGS: [(i32, i32, i32); 6] = [
    (25, 1, 0),
    (50, 2, 0),
    (100, 3, 0),
    (20, 0, 1),
    (40, 0, 2),
    (80, 0, 3),
];

mod game {
    use super::RINGS;

    pub struct Game {
        boss_points: i32,
        boss_damage: i32,
        boss_armor: i32,
        pub won_games: Vec<i32>,
        pub lost_games: Vec<i32>,
    }

    impl Game {
        pub fn new(boss_points: i32, boss_damage: i32, boss_armor: i32) -> Self {
            Self {
                boss_points,
                boss_damage,
                boss_armor,
                won_games: Vec::new(),
                lost_games: Vec::new(),
            }
        }

        fn play2(&mut self, player_points: i32, player_damage: i32, player_armor: i32, cost: i32) {
            let mut player_points = player_points;
            let mut boss_points = self.boss_points;

            while player_points > 0 && boss_points > 0 {
                let pd = (player_damage - self.boss_armor).max(1);
                boss_points -= pd;
                if boss_points <= 0 {
                    break;
                }

                let bd = (self.boss_damage - player_armor).max(1);
                player_points -= bd;
                if player_points <= 0 {
                    break;
                }
            }

            if player_points > 0 {
                self.won_games.push(cost);
            } else {
                self.lost_games.push(cost);
            }
        }

        fn play1(
            &mut self,
            player_points: i32,
            player_damage: i32,
            player_armor: i32,
            cost: i32,
            ring_i1: usize,
        ) {
            for (ring_i2, ring_2) in RINGS.iter().enumerate() {
                if ring_i2 == ring_i1 {
                    continue;
                }
                self.play2(
                    player_points,
                    player_damage + ring_2.1,
                    player_armor + ring_2.2,
                    cost + ring_2.0,
                );
            }
            self.play2(player_points, player_damage, player_armor, cost);
        }

        pub fn play(
            &mut self,
            player_points: i32,
            player_damage: i32,
            player_armor: i32,
            cost: i32,
        ) {
            for (ring_i1, ring_1) in RINGS.iter().enumerate() {
                self.play1(
                    player_points,
                    player_damage + ring_1.1,
                    player_armor + ring_1.2,
                    cost + ring_1.0,
                    ring_i1,
                );
            }
            self.play1(
                player_points,
                player_damage,
                player_armor,
                cost,
                RINGS.len(),
            );
        }
    }
}

fn main() {
    let input = fs::read_to_string("input.txt").expect("Could not read file");
    let mut boss_stats = input.lines().map(|l| {
        let p = l.split_once(": ").unwrap();
        p.1.parse::<i32>().unwrap()
    });

    let player_points = 100;

    let mut game = game::Game::new(
        boss_stats.next().unwrap(),
        boss_stats.next().unwrap(),
        boss_stats.next().unwrap(),
    );

    for weapon in WEAPONS {
        for armor in ARMOR {
            game.play(
                player_points,
                weapon.1 + armor.1,
                weapon.2 + armor.2,
                weapon.0 + armor.0,
            );
        }
        game.play(player_points, weapon.1, weapon.2, weapon.0);
    }

    println!("{}", game.won_games.into_iter().min().unwrap());
    println!("{}", game.lost_games.into_iter().max().unwrap());
}
