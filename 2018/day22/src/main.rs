mod grid;

use crate::grid::Grid;
use std::fs;

#[derive(Clone, Copy, PartialEq, Eq)]
enum Gear {
    Neither = 0,
    Torch = 1,
    Climbing = 2,
}

#[derive(Clone, Copy, PartialEq, Eq)]
struct State {
    minutes: u32,
    x: usize,
    y: usize,
    gear: Gear,
}

fn erosion_level(
    x: usize,
    y: usize,
    target_x: usize,
    target_y: usize,
    depth: usize,
    cache: &mut Grid<u32>,
) -> u32 {
    let cv = cache.get(x, y);
    if cv != u32::MAX {
        cv
    } else {
        let gi = if (y == 0 && x == 0) || (y == target_y && x == target_x) {
            0
        } else if y == 0 {
            x as u32 * 16807
        } else if x == 0 {
            y as u32 * 48271
        } else {
            erosion_level(x - 1, y, target_x, target_y, depth, cache)
                * erosion_level(x, y - 1, target_x, target_y, depth, cache)
        };
        let el = (gi + depth as u32) % 20183;
        cache.insert(x, y, el);
        el
    }
}

fn main() {
    // parse input
    let input = fs::read_to_string("input.txt").expect("Could not read file");
    let mut lines = input.lines();

    let depth = lines.next().unwrap()[7..].parse::<usize>().unwrap();
    let (target_x, target_y) = lines.next().unwrap()[8..].split_once(',').unwrap();
    let target_x = target_x.parse::<usize>().unwrap();
    let target_y = target_y.parse::<usize>().unwrap();

    // part 1
    let mut cache = Grid::new(target_x, target_y, u32::MAX);
    let mut sum = 0;
    for y in 0..=target_y {
        for x in 0..=target_x {
            sum += erosion_level(x, y, target_x, target_y, depth, &mut cache) % 3;
        }
    }
    println!("{sum}");

    // part 2 - Combine A* (instead of Dijkstra's for a more informed search)
    // with Dial's algorithm (n + 1 buckets instead of a BinaryHeap where n is
    // the maximum edge weight, which is 14 in our case, because it may take 7
    // minutes to change gear plus 7 minutes to change it back)
    let mut buckets = vec![Vec::new(); 15];
    let mut best = vec![Grid::new(target_x, target_y, u32::MAX); 3];
    let initial = State {
        minutes: 0,
        x: 0,
        y: 0,
        gear: Gear::Torch,
    };
    best[Gear::Torch as usize].insert(0, 0, 0);
    buckets[0].push(initial);

    let mut bucket = target_x + target_y;
    'outer: loop {
        while let Some(s) = buckets[0].pop() {
            if s.x == target_x && s.y == target_y && s.gear == Gear::Torch {
                println!("{}", s.minutes);
                break 'outer;
            }

            for dir in [(0, 1), (1, 0), (0, -1), (-1, 0)] {
                let nx = s.x as i64 + dir.0;
                let ny = s.y as i64 + dir.1;
                if nx >= 0 && ny >= 0 {
                    let e = best[s.gear as usize].get(nx as usize, ny as usize);
                    if e > s.minutes + 1 {
                        let dest_type = erosion_level(
                            nx as usize,
                            ny as usize,
                            target_x,
                            target_y,
                            depth,
                            &mut cache,
                        ) % 3;
                        if s.gear as u32 != dest_type {
                            // Use manhattan distance as a simple heuristic for
                            // the A* algorithm. Include the time it takes to
                            // change gear.
                            let mut eta =
                                target_x.abs_diff(nx as usize) + target_y.abs_diff(ny as usize);
                            if s.gear != Gear::Torch {
                                eta += 7;
                            }

                            let ns = State {
                                minutes: s.minutes + 1,
                                x: nx as usize,
                                y: ny as usize,
                                gear: s.gear,
                            };
                            best[ns.gear as usize].insert(ns.x, ns.y, ns.minutes);
                            buckets[ns.minutes as usize + eta - bucket].push(ns);
                        }
                    }
                }
            }

            let current_type = erosion_level(s.x, s.y, target_x, target_y, depth, &mut cache) % 3;
            for ng in [Gear::Neither, Gear::Torch, Gear::Climbing] {
                if ng as u32 == current_type {
                    continue;
                }
                let e = best[ng as usize].get(s.x, s.y);
                if e > s.minutes + 7 {
                    let mut eta = target_x.abs_diff(s.x) + target_y.abs_diff(s.y);
                    if ng != Gear::Torch {
                        eta += 7;
                    }

                    let ns = State {
                        minutes: s.minutes + 7,
                        x: s.x,
                        y: s.y,
                        gear: ng,
                    };
                    best[ns.gear as usize].insert(ns.x, ns.y, ns.minutes);
                    buckets[ns.minutes as usize + eta - bucket].push(ns);
                }
            }
        }

        // `rotate` is very fast for 15 elements. As a matter of fact, it's
        // overall slightly faster than using index % length whenever we access
        // the Vec.
        buckets.rotate_left(1);
        bucket += 1;
    }
}
