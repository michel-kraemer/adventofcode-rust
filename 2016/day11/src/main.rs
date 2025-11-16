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

    // slow: consider each configuration individually
    // s.floors.hash(&mut h);

    // VERY fast: only consider how many generators and microchips we have at
    // each level. See the following reddit comment:
    // https://www.reddit.com/r/adventofcode/comments/5hoia9/comment/db1v1ws/
    for f in &s.floors {
        let mut generators = 0;
        let mut microchips = 0;
        for j in f.chunks(2) {
            if j[0] {
                generators += 1;
            }
            if j[1] {
                microchips += 1;
            }
        }
        generators.hash(&mut h);
        microchips.hash(&mut h);
    }

    s.elevator.hash(&mut h);
    h.finish()
}

fn move_one_item(
    s: &State,
    to_floor: usize,
    seen: &mut HashSet<u64>,
    queue: &mut BinaryHeap<Reverse<State>>,
) {
    for i in 0..s.floors[s.elevator].len() {
        if !s.floors[s.elevator][i] {
            continue;
        }
        let mut ns = s.clone();
        ns.steps += 1;
        ns.floors[ns.elevator][i] = false;
        ns.elevator = to_floor;
        ns.floors[ns.elevator][i] = true;
        let ns_key = key(&ns);
        if is_valid(&ns) && !seen.contains(&ns_key) {
            seen.insert(ns_key);
            queue.push(Reverse(ns));
        }
    }
}

fn move_two_items(
    s: &State,
    to_floor: usize,
    seen: &mut HashSet<u64>,
    queue: &mut BinaryHeap<Reverse<State>>,
) {
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
            ns.elevator = to_floor;
            ns.floors[ns.elevator][i] = true;
            ns.floors[ns.elevator][j] = true;
            let ns_key = key(&ns);
            if is_valid(&ns) && !seen.contains(&ns_key) {
                seen.insert(ns_key);
                queue.push(Reverse(ns));
            }
        }
    }
}

fn main() {
    let generator_regex = Regex::new(r"(\w+) generator").unwrap();
    let microchip_regex = Regex::new(r"(\w+)-compatible microchip").unwrap();

    for part1 in [true, false] {
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

            if s.elevator < s.floors.len() - 1 {
                move_one_item(&s, s.elevator + 1, &mut seen, &mut queue);
                move_two_items(&s, s.elevator + 1, &mut seen, &mut queue);
            }
            if s.elevator > 0 {
                move_one_item(&s, s.elevator - 1, &mut seen, &mut queue);
                move_two_items(&s, s.elevator - 1, &mut seen, &mut queue);
            }
        }

        println!("{}", steps);
    }
}
