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

struct Erosion {
    target_x: usize,
    target_y: usize,
    depth: u32,
    init_x: u32,
    init_y: u32,
    cache: Grid<u32>,
}

impl Erosion {
    fn new(target_x: usize, target_y: usize, depth: u32, init_x: u32, init_y: u32) -> Self {
        Self {
            target_x,
            target_y,
            depth,
            init_x,
            init_y,
            cache: Grid::new(target_x, target_y, u32::MAX),
        }
    }

    fn get(&mut self, x: usize, y: usize) -> u32 {
        let cv = self.cache.get(x, y);
        if cv != u32::MAX {
            return cv;
        }

        let gi = if (y == 0 && x == 0) || (y == self.target_y && x == self.target_x) {
            0
        } else if y == 0 {
            x as u32 * self.init_x
        } else if x == 0 {
            y as u32 * self.init_y
        } else {
            self.get(x - 1, y) * self.get(x, y - 1)
        };

        let el = (gi + self.depth) % 20183;
        self.cache.insert(x, y, el);
        el
    }
}

fn main() {
    // parse input
    let input = fs::read_to_string("input.txt").expect("Could not read file");
    let mut lines = input.lines();

    let depth = lines.next().unwrap()[7..].parse::<u32>().unwrap();
    let (target_x, target_y) = lines.next().unwrap()[8..].split_once(',').unwrap();
    let mut target_x = target_x.parse::<usize>().unwrap();
    let mut target_y = target_y.parse::<usize>().unwrap();

    let mut init_x = 16807;
    let mut init_y = 48271;

    if target_y > target_x {
        // Minor performance improvement: this allows us to use resize() more
        // often when updating the grid. See Grid::ensure_size().
        (target_x, target_y) = (target_y, target_x);
        (init_x, init_y) = (init_y, init_x);
    }

    // part 1 ...

    // make the grid a little bit larger than necessary to avoid some
    // reallocations
    let mut erosion = Erosion::new(target_x + 10, target_y + 10, depth, init_x, init_y);
    let mut sum = 0;
    for y in 0..=target_y {
        for x in 0..=target_x {
            sum += erosion.get(x, y) % 3;
        }
    }
    println!("{sum}");

    // part 2 - Combine A* (instead of Dijkstra's for a more informed search)
    // with Dial's algorithm (n + 1 buckets instead of a BinaryHeap where n is
    // the maximum edge weight, which is 14 in our case, because it may take 7
    // minutes to change gear plus 7 minutes to change it back)
    let mut buckets = (0..15)
        .map(|_| Vec::with_capacity(1000))
        .collect::<Vec<_>>();
    let mut best = vec![Grid::new(target_x + 10, target_y + 10, u32::MAX); 3];
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
                        let dest_type = erosion.get(nx as usize, ny as usize) % 3;
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

            let current_type = erosion.get(s.x, s.y) % 3;
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
