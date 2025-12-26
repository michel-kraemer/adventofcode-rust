use std::fs;

#[derive(Clone, Copy, PartialEq, Eq)]
enum Dir {
    Left,
    Right,
}

fn main() {
    let input = fs::read_to_string("input.txt").expect("Could not read file");
    let instructions = input.trim().split(", ");

    let mut x = 0;
    let mut y = 0;
    let mut dir = (0i64, 1i64);

    let mut min_x = i64::MAX;
    let mut min_y = i64::MAX;
    let mut max_x = i64::MIN;
    let mut max_y = i64::MIN;

    // part 1 - Just follow the instructions. Record min_x, min_y, max_x, and
    // max_y in preparation for part 2.
    let mut parsed_instructions = Vec::with_capacity(200);
    for i in instructions {
        let dist = i[1..].parse::<i64>().unwrap();
        if i.starts_with('L') {
            dir = (-dir.1, dir.0);
            parsed_instructions.push((Dir::Left, dist));
        } else {
            dir = (dir.1, -dir.0);
            parsed_instructions.push((Dir::Right, dist));
        };
        x += dir.0 * dist;
        y += dir.1 * dist;

        min_x = min_x.min(x);
        min_y = min_y.min(y);
        max_x = max_x.max(x);
        max_y = max_y.max(y);
    }

    println!("{}", (x + y).abs());

    // part 2 - create empty grid and record all positions we've visited until
    // we visit one again
    let w = (max_x - min_x).abs() + 1;
    let h = (max_y - min_y).abs() + 1;
    let mut grid = vec![false; (w * h) as usize];
    let mut x = -min_x;
    let mut y = -min_y;
    let mut dir = (0i64, 1i64);
    grid[(y * w + x) as usize] = true;

    'outer: for (lr, dist) in parsed_instructions {
        dir = match lr {
            Dir::Left => (-dir.1, dir.0),
            Dir::Right => (dir.1, -dir.0),
        };

        for _ in 0..dist {
            x += dir.0;
            y += dir.1;
            if grid[(y * w + x) as usize] {
                println!("{}", (x + min_x + y + min_y).abs());
                break 'outer;
            }
            grid[(y * w + x) as usize] = true;
        }
    }
}
