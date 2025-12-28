use std::fs;

#[cfg(feature = "visualize")]
use screen::Screen;

fn rotate_down(grid: &mut [[bool; 50]; 6], x: usize, n: usize) {
    assert!(n < grid.len());
    assert!(x < grid[0].len());

    let mut tmp: [bool; 6] = [false; 6];

    let left = grid.len() - n;
    let right = n;

    if left <= right {
        for y in 0..left {
            tmp[y] = grid[y][x];
        }
        for y in 0..right {
            grid[y][x] = grid[left + y][x];
        }
        for y in 0..left {
            grid[right + y][x] = tmp[y];
        }
    } else {
        for y in 0..grid.len() - left {
            tmp[y] = grid[y + left][x];
        }
        for y in (right..grid.len()).rev() {
            grid[y][x] = grid[y - n][x];
        }
        for y in 0..grid.len() - left {
            grid[y][x] = tmp[y];
        }
    }
}

#[cfg(feature = "visualize")]
fn visualize_grid(grid: &[[bool; 50]; 6], screen: &mut Screen) {
    let mut new_grid = [' '; 50 * 6];
    for y in 0..6 {
        for x in 0..50 {
            if grid[y][x] {
                new_grid[y * 50 + x] = '█';
            }
        }
    }
    screen.update(&new_grid);
}

fn main() {
    let input = fs::read_to_string("input.txt").expect("Could not read file");
    let instructions = input.lines().collect::<Vec<_>>();

    #[cfg(feature = "visualize")]
    let mut screen = Screen::new(50, 6, 100);

    let mut grid = [[false; 50]; 6];
    for i in instructions {
        if i.starts_with("rect") {
            let (_, size) = i.split_once(' ').unwrap();
            let (w, h) = size.split_once('x').unwrap();
            let w = w.parse::<usize>().unwrap();
            let h = h.parse::<usize>().unwrap();

            #[cfg(not(feature = "visualize"))]
            for row in grid.iter_mut().take(h) {
                for c in row.iter_mut().take(w) {
                    *c = true;
                }
            }

            #[cfg(feature = "visualize")]
            for y in 0..h {
                for x in 0..w {
                    grid[y][x] = true;
                    visualize_grid(&grid, &mut screen);
                }
            }
        } else if i.starts_with("rotate row") {
            let remainder = &i[13..];
            let (row, len) = remainder.split_once(" by ").unwrap();
            let row = row.parse::<usize>().unwrap();
            let len = len.parse::<usize>().unwrap();

            #[cfg(not(feature = "visualize"))]
            grid[row].rotate_right(len);

            #[cfg(feature = "visualize")]
            for _ in 0..len {
                grid[row].rotate_right(1);
                visualize_grid(&grid, &mut screen);
            }
        } else {
            let remainder = &i[16..];
            let (col, len) = remainder.split_once(" by ").unwrap();
            let col = col.parse::<usize>().unwrap();
            let len = len.parse::<usize>().unwrap();

            #[cfg(not(feature = "visualize"))]
            rotate_down(&mut grid, col, len);

            #[cfg(feature = "visualize")]
            for _ in 0..len {
                rotate_down(&mut grid, col, 1);
                visualize_grid(&grid, &mut screen);
            }
        }
    }

    #[cfg(feature = "visualize")]
    screen.finish();

    let mut result = 0;
    for row in &grid {
        for &c in row {
            if c {
                result += 1;
            }
        }
    }

    println!("{result}");

    #[cfg(not(feature = "visualize"))]
    for r in grid {
        println!(
            "{}",
            r.iter()
                .map(|b| if *b { '█' } else { ' ' })
                .collect::<String>()
        );
    }
}
