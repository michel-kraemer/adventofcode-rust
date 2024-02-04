use std::{collections::HashMap, fs};

enum Dir {
    R,
    L,
    U,
    D,
}

fn main() {
    let input = fs::read_to_string("input.txt").expect("Could not read file");
    let mut wires = input.lines().map(|l| {
        l.split(',')
            .map(|p| {
                let dist = p[1..].parse::<i64>().unwrap();
                let dir = if p.starts_with('R') {
                    Dir::R
                } else if p.starts_with('L') {
                    Dir::L
                } else if p.starts_with('U') {
                    Dir::U
                } else if p.starts_with('D') {
                    Dir::D
                } else {
                    panic!("Unknown direction")
                };
                (dir, dist)
            })
            .collect::<Vec<_>>()
    });

    let wire1 = wires.next().unwrap();
    let wire2 = wires.next().unwrap();

    // draw wire 1
    let mut grid = HashMap::new();
    let mut x = 0i64;
    let mut y = 0i64;
    let mut steps = 0;
    for (dir, dist) in wire1 {
        for _ in 0..dist {
            match dir {
                Dir::R => x += 1,
                Dir::L => x -= 1,
                Dir::U => y -= 1,
                Dir::D => y += 1,
            }
            steps += 1;
            grid.entry((x, y)).or_insert(steps);
        }
    }

    // draw wire 2 and find intersections
    x = 0i64;
    y = 0i64;
    steps = 0;
    let mut min_intersection = i64::MAX;
    let mut min_steps = usize::MAX;
    for (dir, dist) in wire2 {
        for _ in 0..dist {
            match dir {
                Dir::R => x += 1,
                Dir::L => x -= 1,
                Dir::U => y -= 1,
                Dir::D => y += 1,
            }
            steps += 1;
            if let Some(other_steps) = grid.get(&(x, y)) {
                min_intersection = min_intersection.min(x.abs() + y.abs());
                min_steps = min_steps.min(steps + other_steps);
            }
        }
    }

    println!("{}", min_intersection);
    println!("{}", min_steps);
}
