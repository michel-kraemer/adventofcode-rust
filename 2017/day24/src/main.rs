use std::{
    collections::{HashSet, VecDeque},
    fs,
};

struct State {
    e: usize,
    strength: usize,
    seen: HashSet<(usize, usize)>,
}

fn main() {
    let input = fs::read_to_string("input.txt").expect("Could not read file");
    let bridges = input
        .lines()
        .map(|l| {
            let p = l.split_once('/').unwrap();
            (p.0.parse::<usize>().unwrap(), p.1.parse::<usize>().unwrap())
        })
        .collect::<Vec<_>>();

    let mut queue = VecDeque::new();
    queue.push_back(State {
        e: 0,
        strength: 0,
        seen: HashSet::new(),
    });

    let mut max = 0;
    let mut longest = 0;
    let mut longest_max = 0;

    while !queue.is_empty() {
        let s = queue.pop_back().unwrap();

        if s.strength > max {
            max = s.strength;
        }

        if s.seen.len() > longest {
            longest = s.seen.len();
            longest_max = s.strength;
        } else if s.seen.len() == longest && s.strength > longest_max {
            longest_max = s.strength;
        }

        for b in &bridges {
            if (b.0 == s.e || b.1 == s.e) && !s.seen.contains(b) {
                let mut ns = s.seen.clone();
                ns.insert(*b);
                let s = State {
                    e: if s.e == b.1 { b.0 } else { b.1 },
                    strength: s.strength + b.0 + b.1,
                    seen: ns,
                };
                queue.push_back(s);
            }
        }
    }

    // part 1
    println!("{}", max);

    // part 2
    println!("{}", longest_max);
}
