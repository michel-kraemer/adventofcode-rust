use std::{
    cmp::Reverse,
    collections::{hash_map::DefaultHasher, BinaryHeap, HashMap, HashSet},
    fs,
    hash::{Hash, Hasher},
};

use regex::Regex;

#[derive(PartialEq, Eq, Clone, Debug)]
struct State {
    steps: usize,
    elevator: usize,
    floors: [Vec<bool>; 4],
}

impl Ord for State {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.steps.cmp(&other.steps)
    }
}

impl PartialOrd for State {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

fn is_valid(s: &State) -> bool {
    for i in 0..s.floors.len() {
        if i == s.elevator {
            // ignore
            continue;
        }
        let mut unpowered_chips = 0;
        let mut generators = 0;
        for j in s.floors[i].chunks(2) {
            if j[1] && !j[0] {
                unpowered_chips += 1;
            }
            if j[0] {
                generators += 1;
            }
            if unpowered_chips > 0 && generators > 0 {
                return false;
            }
        }
    }
    true
}

fn key(s: &State) -> u64 {
    let mut h = DefaultHasher::new();
    s.floors.hash(&mut h);
    s.elevator.hash(&mut h);
    h.finish()
}

fn main() {
    for part1 in [true, false] {
        let generator_regex = Regex::new(r"(\w+) generator").unwrap();
        let microchip_regex = Regex::new(r"(\w+)-compatible microchip").unwrap();

        let input = fs::read_to_string("input.txt").expect("Could not read file");

        let mut categories: HashMap<&str, usize> = HashMap::new();
        for l in input.lines() {
            for g in generator_regex.captures_iter(l) {
                let (_, [generator]) = g.extract();
                let l = categories.len();
                categories.entry(generator).or_insert(l);
            }
        }

        let (elerium_i, dilithium_i) = if !part1 {
            let elerium_i = categories.len();
            categories.insert("elerium", elerium_i);
            let dilithium_i = categories.len();
            categories.insert("dilithium", dilithium_i);
            (elerium_i, dilithium_i)
        } else {
            (0, 0)
        };

        let num_items = categories.len() * 2;

        let mut floors = [
            vec![false; num_items],
            vec![false; num_items],
            vec![false; num_items],
            vec![false; num_items],
        ];
        for (i, l) in input.lines().enumerate() {
            for g in generator_regex.captures_iter(l) {
                let (_, [generator]) = g.extract();
                let j = categories[generator];
                floors[i][j * 2] = true;
            }
            for m in microchip_regex.captures_iter(l) {
                let (_, [microchip]) = m.extract();
                let j = categories[microchip];
                floors[i][j * 2 + 1] = true;
            }
        }

        if !part1 {
            floors[0][elerium_i * 2] = true;
            floors[0][elerium_i * 2 + 1] = true;
            floors[0][dilithium_i * 2] = true;
            floors[0][dilithium_i * 2 + 1] = true;
        }

        let mut queue = BinaryHeap::new();
        let initial_state = State {
            steps: 0,
            elevator: 0,
            floors: floors.clone(),
        };
        let inital_key = key(&initial_state);
        queue.push(Reverse(initial_state));

        let mut seen = HashSet::new();
        seen.insert(inital_key);

        let mut steps = 0;
        while !queue.is_empty() {
            let s = queue.pop().unwrap().0;

            if s.floors[0].iter().all(|i| !i)
                && s.floors[1].iter().all(|i| !i)
                && s.floors[2].iter().all(|i| !i)
            {
                steps = s.steps;
                break;
            }

            for up in [true, false] {
                let next = if up { s.elevator + 1 } else { s.elevator - 1 };
                if next < s.floors.len() {
                    for i in 0..s.floors[s.elevator].len() {
                        if !s.floors[s.elevator][i] {
                            continue;
                        }
                        let mut ns = s.clone();
                        ns.steps += 1;
                        ns.floors[ns.elevator][i] = false;
                        ns.elevator = next;
                        ns.floors[ns.elevator][i] = true;
                        let ns_key = key(&ns);
                        if !seen.contains(&ns_key) && is_valid(&ns) {
                            seen.insert(ns_key);
                            queue.push(Reverse(ns));
                        }
                    }
                }
            }

            for up in [true, false] {
                let next = if up { s.elevator + 1 } else { s.elevator - 1 };
                if next < s.floors.len() {
                    for i in 0..s.floors[s.elevator].len() {
                        if !s.floors[s.elevator][i] {
                            continue;
                        }
                        for j in i + 1..s.floors[s.elevator].len() {
                            if !s.floors[s.elevator][j] {
                                continue;
                            }
                            let mut ns = s.clone();
                            ns.steps += 1;
                            ns.floors[ns.elevator][i] = false;
                            ns.floors[ns.elevator][j] = false;
                            ns.elevator = next;
                            ns.floors[ns.elevator][i] = true;
                            ns.floors[ns.elevator][j] = true;
                            let ns_key = key(&ns);
                            if !seen.contains(&ns_key) && is_valid(&ns) {
                                seen.insert(ns_key);
                                queue.push(Reverse(ns));
                            }
                        }
                    }
                }
            }
        }

        println!("{}", steps);
    }
}
