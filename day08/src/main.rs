use std::{collections::HashMap, fs};

use num::integer::lcm;
use regex::Regex;

fn main() {
    for part1 in [true, false] {
        let r = Regex::new(r"(...)\s*=\s*\((...)\s*,\s*(...)\)").unwrap();

        let input = fs::read_to_string("input.txt").expect("Could not read file");
        let mut lines = input.lines();

        let instructions = lines.next().unwrap().chars().collect::<Vec<_>>();
        let directions = lines.skip(1);

        let mut map = HashMap::new();
        for d in directions {
            let c = r.captures(d).unwrap();
            let from = c.get(1).unwrap().as_str();
            let left = c.get(2).unwrap().as_str();
            let right = c.get(3).unwrap().as_str();
            map.insert(from, (left, right));
        }

        let lengths = map
            .keys()
            .filter(|k| {
                if part1 {
                    **k == "AAA"
                } else {
                    k.ends_with("A")
                }
            })
            .map(|c| {
                let mut current = *c;
                let mut steps = 0usize;
                'outer: loop {
                    for i in instructions.iter() {
                        current = match i {
                            'L' => map.get(current).unwrap().0,
                            'R' => map.get(current).unwrap().1,
                            _ => panic!("Invalid instruction"),
                        };
                        steps += 1;
                        if part1 {
                            if current == "ZZZ" {
                                break 'outer;
                            }
                        } else {
                            if current.ends_with("Z") {
                                break 'outer;
                            }
                        }
                    }
                }
                steps
            });

        let steps = lengths.reduce(|a, b| lcm(a, b)).unwrap();
        println!("{}", steps);
    }
}
