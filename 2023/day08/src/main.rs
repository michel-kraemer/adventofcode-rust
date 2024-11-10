use num::integer::lcm;
use std::{
    collections::{hash_map::Entry::Vacant, HashMap},
    fs,
};

fn main() {
    let input = fs::read_to_string("input.txt").expect("Could not read file");
    let (instructions, lines) = input.split_once("\n\n").unwrap();
    let instructions = instructions.chars().collect::<Vec<_>>();
    let mut map = HashMap::new();
    for l in lines.lines() {
        let (key, value) = l.split_once(" = ").unwrap();
        let (left, right) = value[1..value.len() - 1].split_once(", ").unwrap();
        map.insert(key, (left, right));
    }

    for part1 in [true, false] {
        let start_nodes = map
            .iter()
            .filter(|&(k, _)| if part1 { *k == "AAA" } else { k.ends_with("A") })
            .map(|(k, _)| *k)
            .collect::<Vec<_>>();
        let mut cycles = Vec::new();
        for sn in start_nodes {
            let mut seen = HashMap::new();
            let mut steps = 0;
            let mut cur = sn;
            loop {
                let instr = instructions[steps % instructions.len()];
                steps += 1;

                cur = match instr {
                    'L' => map.get(cur).unwrap().0,
                    'R' => map.get(cur).unwrap().1,
                    _ => unreachable!(),
                };

                let is_ending_node = if part1 {
                    cur == "ZZZ"
                } else {
                    cur.ends_with("Z")
                };

                if is_ending_node {
                    if let Vacant(e) = seen.entry(cur) {
                        e.insert(steps);
                    } else {
                        let cycle_len = steps - seen.get(&cur).unwrap();

                        // Assume that the length of the cycle equals the
                        // number of steps from the start to `cur`. In this
                        // puzzle, this seems to be true.
                        assert_eq!(cycle_len, *seen.get(&cur).unwrap());

                        cycles.push(cycle_len);
                        break;
                    }
                }
            }
        }

        let min_steps = cycles.into_iter().reduce(lcm).unwrap();

        println!("{}", min_steps);
    }
}
