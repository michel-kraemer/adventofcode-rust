use std::fs;

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
enum Turn {
    Left,
    Straight,
    Right,
}

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
struct Cart {
    id: usize,
    x: usize,
    y: usize,
    dx: isize,
    dy: isize,
    turn: Turn,
}

fn main() {
    let input = fs::read_to_string("input.txt").expect("Could not read file");

    let lines = input.lines().collect::<Vec<_>>();
    let width = lines[0].len();
    let height = lines.len();
    let mut grid = lines.iter().flat_map(|l| l.bytes()).collect::<Vec<_>>();

    let mut carts: Vec<Cart> = Vec::new();
    let mut cart_positions = vec![usize::MAX; width * height];
    for y in 0..height {
        for x in 0..width {
            let b = &mut grid[y * width + x];
            let (dx, dy) = match b {
                b'>' => {
                    *b = b'-';
                    (1, 0)
                }
                b'<' => {
                    *b = b'-';
                    (-1, 0)
                }
                b'v' => {
                    *b = b'|';
                    (0, 1)
                }
                b'^' => {
                    *b = b'|';
                    (0, -1)
                }
                _ => continue,
            };
            cart_positions[y * width + x] = carts.len();
            carts.push(Cart {
                x,
                y,
                dx,
                dy,
                id: carts.len(),
                turn: Turn::Left,
            });
        }
    }
    carts.sort_unstable_by_key(|c| (c.y, c.x));

    let mut first_crashed = false;

    while carts.len() > 1 {
        let mut sci = 0;
        while sci < carts.len() {
            let sc = &mut carts[sci];
            cart_positions[sc.y * width + sc.x] = usize::MAX;

            let b = grid[sc.y * width + sc.x];
            match b {
                b'/' => {
                    (sc.dx, sc.dy) = (-sc.dy, -sc.dx);
                }

                b'\\' => {
                    (sc.dx, sc.dy) = (sc.dy, sc.dx);
                }

                b'+' => {
                    let ndir = match sc.turn {
                        Turn::Left => (sc.dy, -sc.dx, Turn::Straight),
                        Turn::Straight => (sc.dx, sc.dy, Turn::Right),
                        Turn::Right => (-sc.dy, sc.dx, Turn::Left),
                    };
                    sc.dx = ndir.0;
                    sc.dy = ndir.1;
                    sc.turn = ndir.2;
                }

                _ => {
                    // continue straight
                }
            }

            let scx = sc.x.wrapping_add_signed(sc.dx);
            let scy = sc.y.wrapping_add_signed(sc.dy);

            sc.x = scx;
            sc.y = scy;

            let mut crashed = false;
            let other_cart = &mut cart_positions[scy * width + scx];
            if *other_cart != usize::MAX {
                let oci = carts.iter().position(|c| c.id == *other_cart).unwrap();
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
                *other_cart = sc.id;
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

        // `carts` is mostly sorted, so `sort_by_key` is faster than
        // `sort_unstable_by_key`
        carts.sort_by_key(|c| (c.y, c.x));
    }

    // part 2
    println!("{},{}", carts[0].x, carts[0].y);
}
