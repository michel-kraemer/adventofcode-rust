use core::panic;
use std::{
    cmp::Reverse,
    collections::{hash_map::DefaultHasher, BinaryHeap, HashSet},
    fs,
    hash::{Hash, Hasher},
};

#[derive(PartialEq, Eq)]
struct State {
    steps: usize,
    empty_x: i32,
    empty_y: i32,
    goal_x: i32,
    goal_y: i32,
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
    s.empty_x.hash(&mut h);
    s.empty_y.hash(&mut h);
    s.goal_x.hash(&mut h);
    s.goal_y.hash(&mut h);
    h.finish()
}

fn main() {
    let input = fs::read_to_string("input.txt").expect("Could not read file");
    let nodes = input
        .lines()
        .filter(|l| l.starts_with("/dev/grid/"))
        .map(|l| {
            let mut n = l.split(' ').filter(|p| !p.is_empty());
            let mut xy = n.next().unwrap().split('-');
            let x = xy
                .nth(1)
                .unwrap()
                .strip_prefix('x')
                .unwrap()
                .parse::<usize>()
                .unwrap();
            let y = xy
                .next()
                .unwrap()
                .strip_prefix('y')
                .unwrap()
                .parse::<usize>()
                .unwrap();
            let used = n
                .nth(1)
                .unwrap()
                .strip_suffix('T')
                .unwrap()
                .parse::<usize>()
                .unwrap();
            let avail = n
                .next()
                .unwrap()
                .strip_suffix('T')
                .unwrap()
                .parse::<usize>()
                .unwrap();
            (x, y, used, avail)
        })
        .collect::<Vec<_>>();

    // part 1
    let mut viable = 0;
    for i in 0..nodes.len() {
        for j in i + 1..nodes.len() {
            let a = &nodes[i];
            let b = &nodes[j];
            if a.2 != 0 && b.3 >= a.2 {
                viable += 1;
            }
            if b.2 != 0 && a.3 >= b.2 {
                viable += 1;
            }
        }
    }
    println!("{}", viable);

    // part 2
    // assumption A: there is exactly one empty node
    let empty_nodes = nodes.iter().filter(|n| n.2 == 0).count();
    if empty_nodes != 1 {
        panic!("There must be exactly one empty node!");
    }
    let empty = nodes.iter().find(|n| n.2 == 0).unwrap();

    // assumption B: no two neighbors are able to exchange data (except for the
    // neighbors of the empty node)
    let mut viable_neigbors = 0;
    for i in 0..nodes.len() {
        for j in i + 1..nodes.len() {
            let a = &nodes[i];
            let b = &nodes[j];
            if a.0.abs_diff(b.0) != 1 || a.1.abs_diff(b.1) != 1 {
                continue;
            }
            if a == empty || b == empty {
                continue;
            }
            if a.2 != 0 && b.3 >= a.2 {
                viable_neigbors += 1;
            }
            if b.2 != 0 && a.3 >= b.2 {
                viable_neigbors += 1;
            }
        }
    }
    if !viable_neigbors == 0 {
        panic!("No two neighbors must be able to exchange data!");
    }

    // find max x and y
    let max_x = nodes.iter().map(|n| n.0).max().unwrap();
    let max_y = nodes.iter().map(|n| n.1).max().unwrap();

    // convert to grid
    let mut grid_with_sizes = vec![vec![(0, 0); max_x + 1]; max_y + 1];
    for n in &nodes {
        grid_with_sizes[n.1][n.0] = (n.2, n.2 + n.3);
    }

    // assumption C: there are a few nodes whose data cannot be moved to any
    // of their neighbors, all other nodes are interchangable (i.e. the data
    // of each of these nodes would fit on any other node). Mark the nodes that
    // cannot be moved with `false` and the interchangable nodes with `true`.
    let mut grid = vec![vec![false; max_x + 1]; max_y + 1];
    for y in 0..grid_with_sizes.len() {
        for x in 0..grid_with_sizes[y].len() {
            grid[y][x] = [(-1, 0), (1, 0), (0, -1), (0, 1)].iter().all(|dir| {
                let nx = x as i32 + dir.0;
                let ny = y as i32 + dir.1;
                if nx < 0
                    || ny < 0
                    || nx as usize >= grid_with_sizes[y].len()
                    || ny as usize >= grid_with_sizes.len()
                {
                    true
                } else {
                    grid_with_sizes[ny as usize][nx as usize].1 >= grid_with_sizes[y][x].0
                }
            });
        }
    }

    // BFS
    let mut queue = BinaryHeap::new();
    let mut seen = HashSet::new();
    let initial = State {
        steps: 0,
        empty_x: empty.0 as i32,
        empty_y: empty.1 as i32,
        goal_x: max_x as i32,
        goal_y: 0,
    };
    let initial_key = key(&initial);
    queue.push(Reverse(initial));
    seen.insert(initial_key);

    while !queue.is_empty() {
        let s = queue.pop().unwrap().0;

        if s.goal_x == 0 && s.goal_y == 0 {
            println!("{}", s.steps);
            break;
        }

        for dir in [(-1, 0), (1, 0), (0, -1), (0, 1)] {
            let nx = s.empty_x + dir.0;
            let ny = s.empty_y + dir.1;
            if nx >= 0
                && nx < grid[0].len() as i32
                && ny >= 0
                && ny < grid.len() as i32
                && grid[ny as usize][nx as usize]
            {
                let mut ngx = s.goal_x;
                let mut ngy = s.goal_y;
                if ngx == nx && ngy == ny {
                    ngx = s.empty_x;
                    ngy = s.empty_y;
                }
                let ns = State {
                    steps: s.steps + 1,
                    empty_x: nx,
                    empty_y: ny,
                    goal_x: ngx,
                    goal_y: ngy,
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
