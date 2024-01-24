use std::fs;

fn drop(grid: &mut [Vec<char>], ix: usize, iy: usize) {
    let x = ix;
    let mut y = iy;
    grid[y][x] = '|';
    while y < grid.len() - 1 && grid[y + 1][x] != '#' && grid[y + 1][x] != '~' {
        y += 1;
        grid[y][x] = '|';
    }
    if y == grid.len() - 1 {
        return;
    }

    let mut left_x = x;
    while left_x > 0 && grid[y][left_x - 1] != '#' && grid[y][left_x - 1] != '~' {
        left_x -= 1;
        grid[y][left_x] = '|';
        if grid[y + 1][left_x] != '#' && grid[y + 1][left_x] != '~' {
            drop(grid, left_x, y);
            left_x = usize::MAX;
            break;
        }
    }

    let mut right_x = x;
    while right_x < grid[0].len() - 1 && grid[y][right_x + 1] != '#' && grid[y][right_x + 1] != '~'
    {
        right_x += 1;
        grid[y][right_x] = '|';
        if grid[y + 1][right_x] != '#' && grid[y + 1][right_x] != '~' {
            drop(grid, right_x, y);
            right_x = usize::MAX;
            break;
        }
    }

    if left_x != usize::MAX && right_x != usize::MAX {
        if left_x != x {
            grid[y][left_x] = '~';
        } else {
            grid[y][right_x] = '~';
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
            if a.chars().next().unwrap() == 'x' {
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
            min_x = min_x.min(x1 - 1);
            max_x = max_x.max(x2 + 1);
            ((x1, x2), (y1, y2))
        })
        .collect::<Vec<_>>();

    let coords = coords
        .into_iter()
        .map(|c| {
            (
                (c.0 .0 - min_x, c.0 .1 - min_x),
                (c.1 .0 - min_y, c.1 .1 - min_y),
            )
        })
        .collect::<Vec<_>>();

    let mut grid = vec![vec!['.'; max_x - min_x + 1]; max_y - min_y + 1];

    for c in &coords {
        for y in c.1 .0..=c.1 .1 {
            for x in c.0 .0..=c.0 .1 {
                grid[y][x] = '#';
            }
        }
    }

    grid.iter()
        .for_each(|r| println!("{}", String::from_iter(r)));
    println!("{} {} {} {}", min_x, min_y, max_x, max_y);

    let spring_x = 500 - min_x;

    for n in 0..100000 {
        drop(&mut grid, spring_x, 0);

        // grid.iter()
        //     .for_each(|r| println!("{}", String::from_iter(r)));
        // println!("{} {} {} {}", min_x, min_y, max_x, max_y);
        // println!();
        let mut count = 0;
        for y in 0..grid.len() {
            for x in 0..grid[y].len() {
                if grid[y][x] == '~' || grid[y][x] == '|' {
                    count += 1;
                }
            }
        }
        println!("{}", count);
        if count == 44729 {
            break;
        }

        let mut count2 = 0;
        for y in 0..grid.len() {
            for x in 0..grid[y].len() {
                if grid[y][x] == '~' {
                    count2 += 1;
                }
            }
        }
        println!("{}", count2);
    }

    grid.iter()
        .for_each(|r| println!("{}", String::from_iter(r)));
    println!("{} {} {} {}", min_x, min_y, max_x, max_y);
    println!();
}
