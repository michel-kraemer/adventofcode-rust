use std::{fs, vec};

fn fall(grid: &mut [Vec<i32>], brick: ((i32, i32, i32), (i32, i32, i32))) -> i32 {
    let ((x1, y1, z1), (x2, y2, z2)) = brick;
    let mut lowest = 0;
    for x in x1.min(x2)..=x1.max(x2) {
        for y in y1.min(y2)..=y1.max(y2) {
            if grid[x as usize][y as usize] > lowest {
                lowest = grid[x as usize][y as usize];
            }
        }
    }
    for x in x1.min(x2)..=x1.max(x2) {
        for y in y1.min(y2)..=y1.max(y2) {
            grid[x as usize][y as usize] = lowest + (z2 - z1 + 1).abs();
        }
    }
    lowest
}

fn main() {
    let input = fs::read_to_string("input.txt").expect("Could not read file");
    let mut bricks = input
        .lines()
        .map(|l| {
            let (from, to) = l.split_once('~').unwrap();
            let from = from
                .split(",")
                .map(|x| x.parse::<i32>().unwrap())
                .collect::<Vec<_>>();
            let to = to
                .split(",")
                .map(|x| x.parse::<i32>().unwrap())
                .collect::<Vec<_>>();
            ((from[0], from[1], from[2]), (to[0], to[1], to[2]))
        })
        .collect::<Vec<_>>();

    bricks.sort_by(|a, b| {
        let za = a.0 .2.min(a.1 .2);
        let zb = b.0 .2.min(b.1 .2);
        za.cmp(&zb)
    });

    let max_x = bricks.iter().map(|b| b.0 .0.max(b.1 .0)).max().unwrap();
    let max_y = bricks.iter().map(|b| b.0 .1.max(b.1 .1)).max().unwrap();

    let mut initial_grid = vec![vec![0i32; max_x as usize + 1]; max_y as usize + 1];
    let mut initial_tops = Vec::new();
    for b in &bricks {
        initial_tops.push(fall(&mut initial_grid, *b));
    }

    let mut is_safe = 0;
    let mut would_fall = 0;
    for i in 0..bricks.len() {
        let mut grid = vec![vec![0i32; max_x as usize + 1]; max_y as usize + 1];
        let mut did_fall = false;
        for (j, b) in bricks.iter().enumerate() {
            if i == j {
                continue;
            }
            let t = fall(&mut grid, *b);
            if t != initial_tops[j] {
                did_fall = true;
                would_fall += 1;
            }
        }
        if !did_fall {
            is_safe += 1;
        }
    }

    // part 1
    println!("{}", is_safe);

    // part 2
    println!("{}", would_fall);
}
