mod grid;

use crate::grid::Grid;
use std::fs;

#[derive(Clone, Copy, PartialEq, Eq)]
enum Gear {
    Neither = 0,
    Torch = 1,
    Climbing = 2,
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
    best[Gear::Torch as usize].insert(0, 0, 0);
    buckets[0].push((0, 0, Gear::Torch));

    let mut bucket = target_x + target_y;
    'outer: loop {
        while let Some((x, y, gear)) = buckets[0].pop() {
            let current_minutes = best[gear as usize].get(x, y);

            if x == target_x && y == target_y && gear == Gear::Torch {
                println!("{current_minutes}");
                break 'outer;
            }

            for dir in [(0, 1), (1, 0), (0, -1), (-1, 0)] {
                let Some(nx) = x.checked_add_signed(dir.0) else {
                    continue;
                };
                let Some(ny) = y.checked_add_signed(dir.1) else {
                    continue;
                };
                let e = best[gear as usize].get(nx, ny);
                if e > current_minutes + 1 {
                    let dest_type = erosion.get(nx, ny) % 3;
                    if gear as u32 != dest_type {
                        // Use manhattan distance as a simple heuristic for
                        // the A* algorithm. Include the time it takes to
                        // change gear.
                        let mut eta = target_x.abs_diff(nx) + target_y.abs_diff(ny);
                        if gear != Gear::Torch {
                            eta += 7;
                        }

                        let new_minutes = current_minutes + 1;
                        best[gear as usize].insert(nx, ny, new_minutes);
                        buckets[new_minutes as usize + eta - bucket].push((nx, ny, gear));
                    }
                }
            }

            let current_type = erosion.get(x, y) % 3;
            for ng in [Gear::Neither, Gear::Torch, Gear::Climbing] {
                if ng as u32 == current_type {
                    continue;
                }
                let e = best[ng as usize].get(x, y);
                if e > current_minutes + 7 {
                    let mut eta = target_x.abs_diff(x) + target_y.abs_diff(y);
                    if ng != Gear::Torch {
                        eta += 7;
                    }

                    let new_minutes = current_minutes + 7;
                    best[ng as usize].insert(x, y, new_minutes);
                    buckets[new_minutes as usize + eta - bucket].push((x, y, ng));
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
