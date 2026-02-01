use std::{fs, str::Bytes};

#[cfg(feature = "visualize")]
mod visualize;

enum Line {
    Horizontal { y: usize, x: (usize, usize) },
    Vertical { x: usize, y: (usize, usize) },
}

/// This is much faster than using split() and then parse()
fn parse_number(bytes: &mut Bytes) -> usize {
    let mut r = 0;
    for b in bytes {
        if !b.is_ascii_digit() {
            break;
        }
        r *= 10;
        r += (b - b'0') as usize;
    }
    r
}

fn drop(
    grid: &mut [u8],
    width: usize,
    height: usize,
    x: usize,
    start_y: usize,
    water: &mut usize,
    pipes: &mut usize,
) {
    let mut y = start_y;

    // find bottom
    while y < height && grid[y * width + x] == b'.' {
        y += 1;
    }

    // check if we've reached the end of the grid or if we've already been here
    if y != height && grid[y * width + x] != b'|' {
        y -= 1;
        while y >= start_y {
            let yi = y * width;
            let mut overflow = false;

            // find left and right borders
            let mut left_x = x;
            let mut right_x = x;
            for (z, dz, e) in [(&mut left_x, -1, 0), (&mut right_x, 1, width)] {
                while *z != e && grid[yi + z.wrapping_add_signed(dz)] != b'#' {
                    *z = z.wrapping_add_signed(dz);
                    if grid[yi + width + *z] == b'.' {
                        // Overflow. Drop down.
                        drop(grid, width, height, *z, y + 1, water, pipes);
                        if grid[yi + width + *z] != b'~' {
                            // it's only really an overflow if we've not filled
                            // everything below us with water
                            overflow = true;
                            break;
                        }
                    } else if grid[yi + width + *z] == b'|' {
                        // We hit a pipe coming from above. This is also an overflow.
                        overflow = true;
                        break;
                    }
                }
            }

            if overflow {
                // either the left or right has overflown
                if grid[yi + left_x] == b'|' {
                    // We hit a pipe coming from above. Don't count it twice.
                    left_x += 1;
                }
                if grid[yi + right_x] == b'|' {
                    // We hit a pipe coming from above. Don't count it twice.
                    right_x -= 1;
                }

                // fill horizontal cells with pipes
                grid[yi + left_x..=yi + right_x].fill(b'|');
                *pipes += right_x - left_x + 1;
                break;
            }

            // we found a left and right border - fill this row with water
            grid[yi + left_x..=yi + right_x].fill(b'~');
            *water += right_x - left_x + 1;

            // continue one row higher
            y -= 1;
        }
    }

    // fill cells back to start_y with pipes
    if y >= start_y {
        *pipes += y - start_y;
        y -= 1;
        while y > 0 && y >= start_y {
            grid[y * width + x] = b'|';
            y -= 1;
        }
    }
}

fn main() {
    // parse
    let input = fs::read_to_string("input.txt").expect("Could not read file");

    let mut min_x = usize::MAX;
    let mut max_x = usize::MIN;
    let mut min_y = usize::MAX;
    let mut max_y = usize::MIN;
    let mut coords = Vec::new();
    let mut bytes = input.bytes();
    while let Some(first) = bytes.next() {
        let is_vertical = first == b'x';
        bytes.next(); // skip '='

        let a = parse_number(&mut bytes);
        bytes.nth(2); // skip ' y='

        let b = parse_number(&mut bytes);
        bytes.next(); // skip '.'
        let c = parse_number(&mut bytes);

        if is_vertical {
            coords.push(Line::Vertical { x: a, y: (b, c) });
            min_x = min_x.min(a - 1); // leave space
            max_x = max_x.max(a + 1); // leave space
            min_y = min_y.min(b);
            max_y = max_y.max(c);
        } else {
            coords.push(Line::Horizontal { y: a, x: (b, c) });
            min_y = min_y.min(a);
            max_y = max_y.max(a);
            min_x = min_x.min(b - 1); // leave space
            max_x = max_x.max(c + 1); // leave space
        }
    }

    let width = max_x - min_x + 1;
    let height = max_y - min_y + 1;
    let mut grid = vec![b'.'; width * height];

    for c in coords {
        match c {
            Line::Horizontal { y, x } => {
                let yi = (y - min_y) * width;
                grid[yi + x.0 - min_x..=yi + x.1 - min_x].fill(b'#');
            }
            Line::Vertical { x, y } => {
                let x = x - min_x;
                for i in ((y.0 - min_y) * width + x..(y.1 - min_y) * width + x).step_by(width) {
                    grid[i] = b'#';
                }
            }
        }
    }

    let mut water = 0;
    let mut pipes = 0;
    drop(
        &mut grid,
        width,
        height,
        500 - min_x,
        0,
        &mut water,
        &mut pipes,
    );

    #[cfg(feature = "visualize")]
    {
        grid[500 - min_x] = b'|';
        visualize::visualize(&grid, width, height, 500 - min_x);
    }

    // part 1
    println!("{}", water + pipes);

    // part 2
    println!("{}", water);
}
