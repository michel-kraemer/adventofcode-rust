use std::{
    cmp::Reverse,
    collections::{BinaryHeap, HashSet, hash_map::DefaultHasher},
    fs,
    hash::{Hash, Hasher},
};

#[derive(PartialEq, Eq)]
struct State {
    x: i32,
    y: i32,
    steps: usize,
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

fn key(s: &State) -> u64 {
    let mut h = DefaultHasher::new();
    s.x.hash(&mut h);
    s.y.hash(&mut h);
    h.finish()
}

fn is_space(x: i32, y: i32, num: i32) -> bool {
    let r = (x * x + 3 * x + 2 * x * y + y + y * y) + num;
    r.count_ones().is_multiple_of(2)
}

fn main() {
    for part1 in [true, false] {
        let input = fs::read_to_string("input.txt").expect("Could not read file");
        let num = input.trim().parse::<i32>().unwrap();

        let mut seen = HashSet::new();
        let mut queue = BinaryHeap::new();
        let start = State {
            x: 1,
            y: 1,
            steps: 0,
        };
        seen.insert(key(&start));
        queue.push(Reverse(start));

        let mut result = 0;
        while !queue.is_empty() {
            let s = queue.pop().unwrap().0;

            if part1 && s.x == 31 && s.y == 39 {
                result = s.steps;
                break;
            }

            for d in [(1, 0), (0, 1), (-1, 0), (0, -1)] {
                let nx = s.x + d.0;
                let ny = s.y + d.1;
                if nx >= 0 && ny >= 0 && is_space(nx, ny, num) {
                    let n = State {
                        x: nx,
                        y: ny,
                        steps: s.steps + 1,
                    };
                    let k = key(&n);
                    if !seen.contains(&k) && (part1 || n.steps <= 50) {
                        seen.insert(k);
                        queue.push(Reverse(n));
                    }
                }
            }
        }

        if part1 {
            println!("{}", result);
        } else {
            println!("{}", seen.len());
        }
    }
}
