use std::{cmp::max, fs};

#[derive(Clone, PartialEq, Eq)]
struct Brick {
    x: usize,
    y: usize,
    z: usize,
}

fn fall(bricks: &Vec<(Brick, Brick, usize)>) -> Vec<(Brick, Brick, usize)> {
    let mut grid = vec![vec![0; 10]; 10];

    let mut new_bricks = Vec::new();
    for b in bricks {
        let mut top = 0;
        for x in b.0.x..=b.1.x {
            top = max(top, grid[b.0.y][x]);
        }
        for y in b.0.y..=b.1.y {
            top = max(top, grid[y][b.0.x]);
        }

        let h = b.1.z - b.0.z;
        let b = (
            Brick {
                x: b.0.x,
                y: b.0.y,
                z: top + 1,
            },
            Brick {
                x: b.1.x,
                y: b.1.y,
                z: top + 1 + h,
            },
            b.2,
        );

        for x in b.0.x..=b.1.x {
            grid[b.0.y][x] = b.1.z;
        }
        for y in b.0.y..=b.1.y {
            grid[y][b.0.x] = b.1.z;
        }

        new_bricks.push(b);
    }

    new_bricks
}

fn parse_brick(s: &str) -> Brick {
    let a = s
        .split(",")
        .map(|i| i.parse::<usize>().unwrap())
        .collect::<Vec<_>>();
    Brick {
        x: a[0],
        y: a[1],
        z: a[2],
    }
}

fn main() {
    let input = fs::read_to_string("input.txt").expect("Could not read file");
    let mut bricks = input
        .lines()
        .map(|line| line.split_once("~").unwrap())
        .enumerate()
        .map(|(i, (s, e))| {
            let a = parse_brick(s);
            let b = parse_brick(e);
            if a.z <= b.z {
                (a, b, i)
            } else {
                (b, a, i)
            }
        })
        .collect::<Vec<_>>();

    bricks.sort_by(|a, b| a.0.z.cmp(&b.0.z));

    let all_fallen_bricks = fall(&bricks);

    let mut s = 0;
    let mut would_fall = 0;
    for i in 0..bricks.len() {
        let filtered_bricks = bricks
            .iter()
            .filter(|b| b.2 != i)
            .map(|b| b.clone())
            .collect::<Vec<_>>();
        let filtered_fallen_bricks = all_fallen_bricks
            .iter()
            .filter(|b| b.2 != i)
            .map(|b| b.clone())
            .collect::<Vec<_>>();

        let fb = fall(&filtered_bricks);
        if fb == filtered_fallen_bricks {
            s += 1;
        } else {
            for j in 0..bricks.len() {
                if j == i {
                    continue;
                }
                let t = filtered_fallen_bricks.iter().find(|b| b.2 == j).unwrap();
                let u = fb.iter().find(|b| b.2 == j).unwrap();
                if t.0.z != u.0.z {
                    would_fall += 1;
                }
            }
        }
    }

    println!("{}", s);
    println!("{}", would_fall);
}
