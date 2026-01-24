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

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
struct Position {
    id: usize,
    step: usize,
}

fn main() {
    // performance optimization: instead of sorting the carts by their y and x
    // coordinate before each step, we optimistically move them, and if we
    // detect a possible crash, we enter a recovery mode where we try the step
    // again and check if the crash actually happens

    let input = fs::read_to_string("input.txt").expect("Could not read file");

    let lines = input.lines().collect::<Vec<_>>();
    let width = lines[0].len();
    let height = lines.len();
    let mut grid = lines.iter().flat_map(|l| l.bytes()).collect::<Vec<_>>();

    let mut carts: Vec<Cart> = Vec::new();
    let mut new_carts: Vec<Cart> = Vec::new();
    let mut cart_positions = vec![
        Position {
            id: usize::MAX,
            step: usize::MAX
        };
        width * height
    ];
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
            cart_positions[y * width + x] = Position {
                id: carts.len(),
                step: 0,
            };
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

    let mut first_crashed = false;
    let mut step = 1;
    let mut recovery = false;

    'outer: while carts.len() > 1 {
        new_carts.clear();

        let mut sci = 0;
        while sci < carts.len() {
            let mut sc = carts[sci];

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

            let other_cart = &mut cart_positions[scy * width + scx];
            if !recovery && other_cart.step.abs_diff(step) <= 1 {
                // a possible crash was detected - enter recovery mode
                recovery = true;

                // increment `step` by an arbitrary number to invalidate all
                // entries in `cart_positions`
                step += 10;
                for c in &carts {
                    cart_positions[c.y * width + c.x] = Position { id: c.id, step };
                }

                // pretend we start in a fresh new step with a sorted list of
                // carts
                step += 1;
                carts.sort_unstable_by_key(|c| (c.y, c.x));

                continue 'outer;
            } else if recovery
                && ((sc.dx == 1 || sc.dy == 1) && other_cart.step + 1 == step
                    || (sc.dx == -1 || sc.dy == -1) && other_cart.step == step)
            {
                // we're in recovery mode and this is a real crash
                let oci = carts.iter().position(|c| c.id == other_cart.id).unwrap();
                if oci < sci {
                    carts.remove(sci);
                    carts.remove(oci);
                    new_carts.remove(oci);
                    sci -= 1;
                } else {
                    carts.remove(oci);
                    carts.remove(sci);
                }
                other_cart.step = usize::MAX;

                if !first_crashed {
                    // part 1
                    println!("{scx},{scy}");
                    first_crashed = true;
                }
            } else {
                // the new position is free
                other_cart.id = sc.id;
                other_cart.step = step;
                new_carts.push(sc);
                sci += 1;
            }
        }

        step += 1;
        recovery = false;
        (carts, new_carts) = (new_carts, carts);
    }

    // part 2
    println!("{},{}", carts[0].x, carts[0].y);
}
