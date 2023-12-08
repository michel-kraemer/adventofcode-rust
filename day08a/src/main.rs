use std::{fs, collections::HashMap};

use regex::Regex;

fn main() {
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

    let mut sum = 0;
    let mut current = "AAA";
    'outer: loop {
        for i in instructions.iter() {
            current = match i {
                'L' => map.get(current).unwrap().0,
                'R' => map.get(current).unwrap().1,
                _ => panic!("Invalid instruction"),
            };
            sum += 1;
            if current == "ZZZ" {
                break 'outer
            }
        }
    }

    println!("{}", sum);
}
