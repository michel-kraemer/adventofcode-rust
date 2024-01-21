use std::{fs, mem};

#[derive(PartialEq, Eq, PartialOrd, Ord, Copy, Clone)]
enum Turn {
    Left,
    Straight,
    Right,
}

#[allow(clippy::needless_range_loop)]
fn main() {
    let input = fs::read_to_string("input.txt").expect("Could not read file");
    let mut grid = input
        .lines()
        .map(|l| l.chars().collect::<Vec<_>>())
        .collect::<Vec<_>>();

    let mut carts: Vec<(i32, i32, i32, i32, Turn)> = Vec::new();
    for y in 0..grid.len() {
        for x in 0..grid[y].len() {
            let c = grid[y][x];
            match c {
                '>' => {
                    grid[y][x] = '-';
                    carts.push((y as i32, x as i32, 0, 1, Turn::Left));
                }
                '<' => {
                    grid[y][x] = '-';
                    carts.push((y as i32, x as i32, 0, -1, Turn::Left));
                }
                'v' => {
                    grid[y][x] = '|';
                    carts.push((y as i32, x as i32, 1, 0, Turn::Left));
                }
                '^' => {
                    grid[y][x] = '|';
                    carts.push((y as i32, x as i32, -1, 0, Turn::Left));
                }
                _ => {}
            }
        }
    }
    carts.sort();

    let mut first_crashed = false;

    while carts.len() > 1 {
        let mut sci = 0;
        while sci < carts.len() {
            let sc = &mut carts[sci];

            let c = grid[sc.0 as usize][sc.1 as usize];
            match c {
                '-' | '|' => {
                    // continue straight
                }

                '/' => {
                    let dy = sc.2;
                    let dx = sc.3;
                    sc.2 = -dx;
                    sc.3 = -dy;
                }

                '\\' => {
                    mem::swap(&mut sc.2, &mut sc.3);
                }

                '+' => {
                    let ndir = match sc.4 {
                        Turn::Left => (-sc.3, sc.2, Turn::Straight),
                        Turn::Straight => (sc.2, sc.3, Turn::Right),
                        Turn::Right => (sc.3, -sc.2, Turn::Left),
                    };
                    sc.2 = ndir.0;
                    sc.3 = ndir.1;
                    sc.4 = ndir.2;
                }

                _ => panic!(),
            }

            sc.0 += sc.2;
            sc.1 += sc.3;

            let scy = sc.0;
            let scx = sc.1;

            let mut crashed = false;
            for j in 0..carts.len() {
                if j == sci {
                    continue;
                }
                let nc = carts[j];
                if nc.0 == scy && nc.1 == scx {
                    if j < sci {
                        carts.remove(j);
                        sci -= 1;
                        carts.remove(sci);
                    } else {
                        carts.remove(sci);
                        carts.remove(j - 1);
                    }
                    crashed = true;
                    break;
                }
            }

            if crashed && !first_crashed {
                // part 1
                println!("{},{}", scx, scy);
                first_crashed = true;
            }

            if !crashed {
                sci += 1;
            }
        }

        carts.sort();
    }

    // part 2
    println!("{},{}", carts[0].1, carts[0].0);
}
