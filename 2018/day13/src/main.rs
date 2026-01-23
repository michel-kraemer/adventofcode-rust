use std::{fs, mem};

#[derive(PartialEq, Eq, Copy, Clone)]
enum Turn {
    Left,
    Straight,
    Right,
}

fn main() {
    let input = fs::read_to_string("input.txt").expect("Could not read file");
    let mut grid = input
        .lines()
        .map(|l| l.chars().collect::<Vec<_>>())
        .collect::<Vec<_>>();

    let mut carts: Vec<(i32, i32, i32, i32, usize, Turn)> = Vec::new();
    let mut cart_positions = vec![vec![usize::MAX; grid[0].len()]; grid.len()];
    for (y, row) in grid.iter_mut().enumerate() {
        for (x, c) in row.iter_mut().enumerate() {
            match c {
                '>' => {
                    *c = '-';
                    cart_positions[y][x] = carts.len();
                    carts.push((y as i32, x as i32, 0, 1, carts.len(), Turn::Left));
                }
                '<' => {
                    *c = '-';
                    cart_positions[y][x] = carts.len();
                    carts.push((y as i32, x as i32, 0, -1, carts.len(), Turn::Left));
                }
                'v' => {
                    *c = '|';
                    cart_positions[y][x] = carts.len();
                    carts.push((y as i32, x as i32, 1, 0, carts.len(), Turn::Left));
                }
                '^' => {
                    *c = '|';
                    cart_positions[y][x] = carts.len();
                    carts.push((y as i32, x as i32, -1, 0, carts.len(), Turn::Left));
                }
                _ => {}
            }
        }
    }
    carts.sort_unstable_by_key(|c| (c.0, c.1));

    let mut first_crashed = false;

    while carts.len() > 1 {
        let mut sci = 0;
        while sci < carts.len() {
            let sc = &mut carts[sci];
            cart_positions[sc.0 as usize][sc.1 as usize] = usize::MAX;

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
                    let ndir = match sc.5 {
                        Turn::Left => (-sc.3, sc.2, Turn::Straight),
                        Turn::Straight => (sc.2, sc.3, Turn::Right),
                        Turn::Right => (sc.3, -sc.2, Turn::Left),
                    };
                    sc.2 = ndir.0;
                    sc.3 = ndir.1;
                    sc.5 = ndir.2;
                }

                _ => panic!(),
            }

            sc.0 += sc.2;
            sc.1 += sc.3;

            let scy = sc.0;
            let scx = sc.1;

            let mut crashed = false;
            let other_cart = &mut cart_positions[scy as usize][scx as usize];
            if *other_cart != usize::MAX {
                let oci = carts.iter().position(|c| c.4 == *other_cart).unwrap();
                if oci < sci {
                    carts.remove(sci);
                    carts.remove(oci);
                    sci -= 1;
                } else {
                    carts.remove(oci);
                    carts.remove(sci);
                }
                crashed = true;
                *other_cart = usize::MAX;
            } else {
                *other_cart = sc.4;
            }

            if crashed && !first_crashed {
                // part 1
                println!("{scx},{scy}");
                first_crashed = true;
            }

            if !crashed {
                sci += 1;
            }
        }

        carts.sort_unstable_by_key(|c| (c.0, c.1));
    }

    // part 2
    println!("{},{}", carts[0].1, carts[0].0);
}
