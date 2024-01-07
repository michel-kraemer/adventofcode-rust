use std::{collections::HashMap, fs};

enum Move {
    S(usize),
    X(usize, usize),
    P(char, char),
}

fn main() {
    for part1 in [true, false] {
        let input = fs::read_to_string("input.txt").expect("Could not read file");
        let moves = input
            .trim()
            .split(',')
            .map(|m| {
                if let Some(m) = m.strip_prefix('s') {
                    Move::S(m.parse().unwrap())
                } else if let Some(m) = m.strip_prefix('x') {
                    let (a, b) = m.split_once('/').unwrap();
                    Move::X(a.parse().unwrap(), b.parse().unwrap())
                } else if let Some(m) = m.strip_prefix('p') {
                    let mut c = m.chars();
                    let a = c.next().unwrap();
                    let b = c.nth(1).unwrap();
                    Move::P(a, b)
                } else {
                    panic!()
                }
            })
            .collect::<Vec<_>>();

        let mut programs = (b'a'..=b'p').map(|b| b as char).collect::<Vec<_>>();

        let mut seen = HashMap::new();
        seen.insert(programs.clone(), 0);

        let result;
        let mut round = 1;
        loop {
            for m in &moves {
                match m {
                    Move::S(len) => {
                        let (a, b) = programs.split_at(programs.len() - len);
                        let mut nm = Vec::from_iter(b.iter().copied());
                        nm.extend(a);
                        programs = nm;
                    }
                    Move::X(a, b) => {
                        programs.swap(*a, *b);
                    }
                    Move::P(a, b) => {
                        let a = programs.iter().position(|p| p == a).unwrap();
                        let b = programs.iter().position(|p| p == b).unwrap();
                        programs.swap(a, b);
                    }
                }
            }

            if part1 {
                result = programs;
                break;
            } else if let Some(cycle_start) = seen.get(&programs) {
                let cycle_len = round - cycle_start;
                round = (1_000_000_000 - cycle_start) % cycle_len + cycle_start;
                result = seen.into_iter().find(|(_, r)| *r == round).unwrap().0;
                break;
            } else {
                seen.insert(programs.clone(), round);
                round += 1;
            }
        }

        println!("{}", String::from_iter(result));
    }
}
