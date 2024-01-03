use std::{
    cmp::Reverse,
    collections::{hash_map::DefaultHasher, BTreeSet, BinaryHeap, HashSet},
    fs,
    hash::{Hash, Hasher},
};

#[derive(PartialEq, Eq)]
struct State {
    steps: usize,
    x: i32,
    y: i32,
    digits_found: BTreeSet<char>,
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
    s.digits_found.hash(&mut h);
    h.finish()
}

fn main() {
    for part1 in [true, false] {
        let input = fs::read_to_string("input.txt").expect("Could not read file");

        let grid = input
            .lines()
            .map(|l| l.chars().collect::<Vec<_>>())
            .collect::<Vec<_>>();

        let mut digits = Vec::new();
        let mut start_x = 0;
        let mut start_y = 0;
        for y in 0..grid.len() {
            for x in 0..grid[0].len() {
                if grid[y][x].is_ascii_digit() {
                    if grid[y][x] == '0' {
                        start_x = x;
                        start_y = y;
                    }
                    digits.push(grid[y][x]);
                }
            }
        }

        let mut seen = HashSet::new();
        let mut queue = BinaryHeap::new();
        let initial = State {
            steps: 0,
            x: start_x as i32,
            y: start_y as i32,
            digits_found: BTreeSet::from(['0']),
        };
        let initial_key = key(&initial);
        seen.insert(initial_key);
        queue.push(Reverse(initial));

        while !queue.is_empty() {
            let s = queue.pop().unwrap().0;

            if s.digits_found.len() == digits.len()
                && (part1 || (s.x as usize == start_x && s.y as usize == start_y))
            {
                println!("{}", s.steps);
                break;
            }

            for dir in [(-1, 0), (1, 0), (0, -1), (0, 1)] {
                let nx = s.x + dir.0;
                let ny = s.y + dir.1;
                let c = grid[ny as usize][nx as usize];
                if c != '#' {
                    let mut ndf = s.digits_found.clone();
                    if c.is_ascii_digit() && !ndf.contains(&c) {
                        ndf.insert(c);
                    }
                    let ns = State {
                        steps: s.steps + 1,
                        x: nx,
                        y: ny,
                        digits_found: ndf,
                    };
                    let k = key(&ns);
                    if !seen.contains(&k) {
                        seen.insert(k);
                        queue.push(Reverse(ns));
                    }
                }
            }
        }
    }
}
