use std::fs;

fn drop(grid: &mut [Vec<char>], x: usize, mut y: usize) {
    if grid[y][x] == '|' {
        // we've already been here
        return;
    }

    // find bottom
    while y < grid.len() && grid[y][x] != '#' && grid[y][x] != '~' {
        grid[y][x] = '|';
        y += 1;
    }
    if y == grid.len() {
        // we've reached the end of the grid
        return;
    }
    y -= 1;

    loop {
        // find left border
        let mut left_x = x;
        while left_x > 0 && grid[y][left_x - 1] != '#' && grid[y][left_x - 1] != '~' {
            left_x -= 1;
            grid[y][left_x] = '|';
            if grid[y + 1][left_x] != '#' && grid[y + 1][left_x] != '~' {
                // overflow. drop down.
                drop(grid, left_x, y + 1);
                left_x = usize::MAX;
                break;
            }
        }

        // find right border
        let mut right_x = x;
        while right_x < grid[0].len() - 1
            && grid[y][right_x + 1] != '#'
            && grid[y][right_x + 1] != '~'
        {
            right_x += 1;
            grid[y][right_x] = '|';
            if grid[y + 1][right_x] != '#' && grid[y + 1][right_x] != '~' {
                // overflow. drop down.
                drop(grid, right_x, y + 1);
                right_x = usize::MAX;
                break;
            }
        }

        if left_x == usize::MAX || right_x == usize::MAX {
            // either the left or right has has overflown
            break;
        }

        // we found a left and right border - fill this row
        for xi in left_x..=right_x {
            grid[y][xi] = '~';
        }

        // continue one row higher
        y -= 1;
        grid[y][x] = '|';
    }
}

fn main() {
    // parse
    let input = fs::read_to_string("input.txt").expect("Could not read file");

    let mut min_x = usize::MAX;
    let mut max_x = usize::MIN;
    let mut min_y = usize::MAX;
    let mut max_y = usize::MIN;
    let coords = input
        .lines()
        .map(|l| {
            let (a, b) = l.split_once(", ").unwrap();
            let a1 = a[2..].parse::<usize>().unwrap();
            let (b1, b2) = b.split_once("..").unwrap();
            let b1 = b1[2..].parse::<usize>().unwrap();
            let b2 = b2.parse::<usize>().unwrap();

            let x1;
            let x2;
            let y1;
            let y2;
            if a.starts_with('x') {
                x1 = a1;
                x2 = a1;
                y1 = b1;
                y2 = b2;
            } else {
                x1 = b1;
                x2 = b2;
                y1 = a1;
                y2 = a1;
            }

            min_y = min_y.min(y1);
            max_y = max_y.max(y2);
            min_x = min_x.min(x1 - 1); // leave space
            max_x = max_x.max(x2 + 1); // leave space

            (x1, x2, y1, y2)
        })
        .collect::<Vec<_>>();

    let mut grid = vec![vec!['.'; max_x - min_x + 1]; max_y - min_y + 1];

    for c in coords {
        for y in c.2..=c.3 {
            for x in c.0..=c.1 {
                grid[y - min_y][x - min_x] = '#';
            }
        }
    }

    drop(&mut grid, 500 - min_x, 0);

    let mut water = 0;
    let mut pipes = 0;

    #[allow(clippy::needless_range_loop)]
    for y in 0..grid.len() {
        for x in 0..grid[y].len() {
            if grid[y][x] == '~' {
                water += 1;
            } else if grid[y][x] == '|' {
                pipes += 1;
            }
        }
    }

    // part 1
    println!("{}", water + pipes);

    // part 2
    println!("{}", water);
}
