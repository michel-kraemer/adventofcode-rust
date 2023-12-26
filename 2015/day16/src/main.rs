use std::{collections::HashMap, fs};

fn main() {
    for part1 in [true, false] {
        let input = fs::read_to_string("input.txt").expect("Could not read file");

        let sues = input
            .lines()
            .map(|l| {
                let (_, attrs) = l.split_once(": ").unwrap();
                attrs
                    .split(", ")
                    .map(|a| {
                        let (k, v) = a.split_once(": ").unwrap();
                        (k, v.parse::<i32>().unwrap())
                    })
                    .collect::<HashMap<_, _>>()
            })
            .collect::<Vec<_>>();

        let what_i_remember = [
            ("children", 3),
            ("cats", 7),
            ("samoyeds", 2),
            ("pomeranians", 3),
            ("akitas", 0),
            ("vizslas", 0),
            ("goldfish", 5),
            ("trees", 3),
            ("cars", 2),
            ("perfumes", 1),
        ];

        let r = sues
            .iter()
            .enumerate()
            .map(|(i, s)| {
                let mut matches = 0;
                for p in &what_i_remember {
                    if let Some(v) = s.get(p.0) {
                        let m = if part1 {
                            *v == p.1
                        } else {
                            match p.0 {
                                "cats" | "trees" => *v > p.1,
                                "pomeranians" | "goldfish" => *v < p.1,
                                _ => *v == p.1,
                            }
                        };
                        if m {
                            matches += 1;
                        }
                    }
                }
                (i, matches)
            })
            .max_by_key(|e| e.1)
            .unwrap();

        println!("{}", r.0 + 1);
    }
}
