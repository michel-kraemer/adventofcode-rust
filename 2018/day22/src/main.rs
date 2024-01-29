use std::{cmp::Reverse, collections::BinaryHeap, fs};

use rustc_hash::FxHashMap;

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
enum Gear {
    Neither,
    Torch,
    Climbing,
}

const ROCKY: usize = 0;
const WET: usize = 1;
const NARROW: usize = 2;

#[derive(Hash, PartialEq, Eq)]
struct Key {
    x: i32,
    y: i32,
    gear: Gear,
}

#[derive(Clone, PartialEq, Eq)]
struct State {
    minutes: usize,
    x: i32,
    y: i32,
    gear: Gear,
}

impl Ord for State {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.minutes.cmp(&other.minutes)
    }
}

impl PartialOrd for State {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

impl State {
    fn to_key(&self) -> Key {
        Key {
            x: self.x,
            y: self.y,
            gear: self.gear,
        }
    }
}

fn erosion_level(
    x: usize,
    y: usize,
    target_x: usize,
    target_y: usize,
    depth: usize,
    cache: &mut FxHashMap<(usize, usize), usize>,
) -> usize {
    if let Some(&e) = cache.get(&(x, y)) {
        e
    } else {
        let gi = if (y == 0 && x == 0) || (y == target_y && x == target_x) {
            0
        } else if y == 0 {
            x * 16807
        } else if x == 0 {
            y * 48271
        } else {
            erosion_level(x - 1, y, target_x, target_y, depth, cache)
                * erosion_level(x, y - 1, target_x, target_y, depth, cache)
        };
        let el = (gi + depth) % 20183;
        cache.insert((x, y), el);
        el
    }
}

fn gear_ok(gear: Gear, tpe: usize) -> bool {
    use Gear::*;

    match gear {
        Neither => tpe == WET || tpe == NARROW,
        Torch => tpe == ROCKY || tpe == NARROW,
        Climbing => tpe == ROCKY || tpe == WET,
    }
}

fn main() {
    use Gear::*;

    // parse input
    let input = fs::read_to_string("input.txt").expect("Could not read file");
    let mut lines = input.lines();

    let depth = lines
        .next()
        .unwrap()
        .split_once(' ')
        .unwrap()
        .1
        .parse::<usize>()
        .unwrap();
    let (target_x, target_y) = lines
        .next()
        .unwrap()
        .split_once(' ')
        .unwrap()
        .1
        .split_once(',')
        .unwrap();
    let target_x = target_x.parse::<usize>().unwrap();
    let target_y = target_y.parse::<usize>().unwrap();

    // part 1
    let mut cache = FxHashMap::default();
    let mut sum = 0;
    #[allow(clippy::needless_range_loop)]
    for y in 0..=target_y {
        for x in 0..=target_x {
            sum += erosion_level(x, y, target_x, target_y, depth, &mut cache) % 3;
        }
    }
    println!("{}", sum);

    // part 2
    let mut queue = BinaryHeap::new();
    let mut seen = FxHashMap::default();
    let initial = State {
        minutes: 0,
        x: 0,
        y: 0,
        gear: Torch,
    };
    seen.insert(initial.to_key(), initial.clone());
    queue.push(Reverse(initial));

    let mut min_minutes = usize::MAX;
    while !queue.is_empty() {
        let s = queue.pop().unwrap().0;

        if s.x as usize == target_x && s.y as usize == target_y {
            let mut minutes = s.minutes;
            if s.gear != Torch {
                minutes += 7;
            }
            min_minutes = min_minutes.min(minutes);
            // do not break here, there might be shorter paths (where we
            // don't have to switch gear)
        }

        if s.minutes > min_minutes {
            // no other path can be shorter
            break;
        }

        for dir in [(0, 1), (1, 0), (0, -1), (-1, 0)] {
            let nx = s.x + dir.0;
            let ny = s.y + dir.1;
            if nx >= 0 && ny >= 0 {
                let dest_type = erosion_level(
                    nx as usize,
                    ny as usize,
                    target_x,
                    target_y,
                    depth,
                    &mut cache,
                ) % 3;
                if gear_ok(s.gear, dest_type) {
                    let ns = State {
                        minutes: s.minutes + 1,
                        x: nx,
                        y: ny,
                        gear: s.gear,
                    };
                    let k = ns.to_key();
                    let e = seen.get(&k);
                    if e.is_none() || e.unwrap().minutes > ns.minutes {
                        seen.insert(k, ns.clone());
                        queue.push(Reverse(ns));
                    }
                }

                for ng in [Neither, Torch, Climbing] {
                    let current_type = erosion_level(
                        s.x as usize,
                        s.y as usize,
                        target_x,
                        target_y,
                        depth,
                        &mut cache,
                    ) % 3;
                    if !gear_ok(ng, current_type) {
                        continue;
                    }

                    let ns = State {
                        minutes: s.minutes + 7,
                        x: s.x,
                        y: s.y,
                        gear: ng,
                    };
                    let k = ns.to_key();
                    let e = seen.get(&k);
                    if e.is_none() || e.unwrap().minutes > ns.minutes {
                        seen.insert(k, ns.clone());
                        queue.push(Reverse(ns));
                    }
                }
            }
        }
    }

    println!("{}", min_minutes);
}
