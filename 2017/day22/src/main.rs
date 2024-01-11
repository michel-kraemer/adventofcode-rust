use std::{collections::HashMap, fs};

enum State {
    Clean,
    Weakened,
    Infected,
    Flagged,
}

fn main() {
    for part1 in [true, false] {
        let input = fs::read_to_string("input.txt").expect("Could not read file");
        let grid = input
            .lines()
            .map(|l| l.chars().collect::<Vec<_>>())
            .collect::<Vec<_>>();

        let mut map = HashMap::new();
        for y in 0..grid.len() {
            for x in 0..grid[0].len() {
                if grid[y][x] == '#' {
                    map.insert((x as i64, y as i64), State::Infected);
                }
            }
        }

        let mut dir = (0, -1);
        let mut virus = ((grid[0].len() / 2) as i64, (grid.len() / 2) as i64);

        let mut infects = 0;
        for _ in 0..(if part1 { 10_000 } else { 10_000_000 }) {
            let s = map.entry(virus).or_insert(State::Clean);
            match s {
                State::Clean => {
                    dir = (dir.1, -dir.0);
                    *s = if part1 {
                        infects += 1;
                        State::Infected
                    } else {
                        State::Weakened
                    };
                }
                State::Weakened => {
                    infects += 1;
                    *s = State::Infected;
                }
                State::Infected => {
                    dir = (-dir.1, dir.0);
                    *s = if part1 { State::Clean } else { State::Flagged };
                }
                State::Flagged => {
                    dir = (-dir.0, -dir.1);
                    *s = State::Clean;
                }
            }
            virus.0 += dir.0;
            virus.1 += dir.1;
        }

        println!("{}", infects);
    }
}
