mod grid;

use crate::grid::Grid;
use std::{cmp::Reverse, collections::BinaryHeap, fs};

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
enum Gear {
    Neither = 0,
    Torch = 1,
    Climbing = 2,
}

const ROCKY: usize = 0;
const WET: usize = 1;
const NARROW: usize = 2;

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

fn erosion_level(
    x: usize,
    y: usize,
    target_x: usize,
    target_y: usize,
    depth: usize,
    cache: &mut Grid<usize>,
) -> usize {
    let cv = cache.get(x, y);
    if cv != usize::MAX {
        cv
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
        cache.insert(x, y, el);
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
    let mut cache = Grid::new(target_x, target_y, usize::MAX);
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
    let mut seen = vec![Grid::new(target_x, target_y, usize::MAX); 3];
    let initial = State {
        minutes: 0,
        x: 0,
        y: 0,
        gear: Torch,
    };
    seen[Torch as usize].insert(0, 0, 0);
    queue.push(Reverse(initial));

    let mut min_minutes = usize::MAX;
    while !queue.is_empty() {
        let s = queue.pop().unwrap().0;

        if s.x as usize == target_x && s.y as usize == target_y && s.gear == Torch {
            min_minutes = s.minutes;
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
                    let e = seen[ns.gear as usize].get(ns.x as usize, ns.y as usize);
                    if e > ns.minutes {
                        seen[ns.gear as usize].insert(ns.x as usize, ns.y as usize, ns.minutes);
                        queue.push(Reverse(ns));
                    }
                }
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
            if gear_ok(ng, current_type) {
                let ns = State {
                    minutes: s.minutes + 7,
                    x: s.x,
                    y: s.y,
                    gear: ng,
                };
                let e = seen[ns.gear as usize].get(ns.x as usize, ns.y as usize);
                if e > ns.minutes {
                    seen[ns.gear as usize].insert(ns.x as usize, ns.y as usize, ns.minutes);
                    queue.push(Reverse(ns));
                }
            }
        }
    }

    println!("{}", min_minutes);
}
