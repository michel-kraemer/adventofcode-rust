use core::panic;
use std::{collections::VecDeque, fs};

use rustc_hash::FxHashSet;

#[derive(PartialEq, Eq, Clone, Copy, Hash)]
struct State {
    empty_x: i32,
    empty_y: i32,
    goal_x: i32,
    goal_y: i32,
}

fn main() {
    let input = fs::read_to_string("input.txt").expect("Could not read file");
    let nodes = input
        .lines()
        .skip(2)
        .map(|l| {
            let mut n = l.split_ascii_whitespace();
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
    for (i, a) in nodes.iter().enumerate() {
        for b in nodes.iter().skip(i + 1) {
            if a.2 != 0 && b.3 >= a.2 {
                viable += 1;
            }
            if b.2 != 0 && a.3 >= b.2 {
                viable += 1;
            }
        }
    }
    println!("{viable}");

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
    for (i, a) in nodes.iter().enumerate() {
        if a == empty {
            continue;
        }
        for b in nodes.iter().skip(i + 1) {
            if b == empty {
                continue;
            }
            if a.0.abs_diff(b.0) != 1 || a.1.abs_diff(b.1) != 1 {
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
    // of their neighbors, all other nodes are interchangeable (i.e. the data
    // of each of these nodes would fit on any other node). Mark the nodes that
    // cannot be moved with `false` and the interchangeable nodes with `true`.
    let mut grid = vec![vec![false; max_x + 1]; max_y + 1];
    for (y, row) in grid_with_sizes.iter().enumerate() {
        for (x, cell) in row.iter().enumerate() {
            grid[y][x] = [(-1, 0), (1, 0), (0, -1), (0, 1)].iter().all(|dir| {
                let nx = x as i32 + dir.0;
                let ny = y as i32 + dir.1;
                if nx < 0
                    || ny < 0
                    || nx as usize >= row.len()
                    || ny as usize >= grid_with_sizes.len()
                {
                    true
                } else {
                    grid_with_sizes[ny as usize][nx as usize].1 >= cell.0
                }
            });
        }
    }

    // BFS
    let mut queue = VecDeque::new();
    let mut seen = FxHashSet::default();
    let initial = State {
        empty_x: empty.0 as i32,
        empty_y: empty.1 as i32,
        goal_x: max_x as i32,
        goal_y: 0,
    };
    queue.push_back((0, initial));
    seen.insert(initial);

    while let Some((steps, s)) = queue.pop_front() {
        if s.goal_x == 0 && s.goal_y == 0 {
            println!("{steps}");
            break;
        }

        for dir in [(-1, 0), (1, 0), (0, -1), (0, 1)] {
            let nx = s.empty_x + dir.0;
            let ny = s.empty_y + dir.1;
            if nx >= 0
                && ny >= 0
                && nx < grid[0].len() as i32
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
                    empty_x: nx,
                    empty_y: ny,
                    goal_x: ngx,
                    goal_y: ngy,
                };
                if seen.insert(ns) {
                    queue.push_back((steps + 1, ns));
                }
            }
        }
    }
}
