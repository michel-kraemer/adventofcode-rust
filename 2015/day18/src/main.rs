use std::fs;

fn count_neighbors(grid: &[Vec<char>], x: usize, y: usize) -> usize {
    let min_x = if x > 0 { x - 1 } else { 0 };
    let max_x = if x < grid[0].len() - 1 {
        x + 1
    } else {
        grid[0].len() - 1
    };
    let min_y = if y > 0 { y - 1 } else { 0 };
    let max_y = if y < grid.len() - 1 {
        y + 1
    } else {
        grid.len() - 1
    };
    let mut result = 0;
    for (yi, row) in grid.iter().enumerate().take(max_y + 1).skip(min_y) {
        for (xi, c) in row.iter().enumerate().take(max_x + 1).skip(min_x) {
            if !(xi == x && yi == y) && *c == '#' {
                result += 1;
            }
        }
    }
    result
}

fn main() {
    for part1 in [true, false] {
        let input = fs::read_to_string("input.txt").expect("Could not read file");

        let mut grid = input
            .lines()
            .map(|c| c.chars().collect::<Vec<_>>())
            .collect::<Vec<_>>();

        let h = grid.len();
        let w = grid[0].len();
        if !part1 {
            grid[0][0] = '#';
            grid[h - 1][0] = '#';
            grid[0][w - 1] = '#';
            grid[h - 1][w - 1] = '#';
        }

        for _ in 0..100 {
            let mut new_grid = grid.clone();
            for y in 0..grid.len() {
                for x in 0..grid[0].len() {
                    if !(part1 || x != 0 && x != w - 1 || y != 0 && y != h - 1) {
                        continue;
                    }

                    let mut on = grid[y][x] == '#';
                    let neighbors = count_neighbors(&grid, x, y);
                    if on && !(2..=3).contains(&neighbors) {
                        on = false;
                    } else if !on && neighbors == 3 {
                        on = true;
                    }
                    if on {
                        new_grid[y][x] = '#';
                    } else {
                        new_grid[y][x] = '.';
                    }
                }
            }
            grid = new_grid;
        }

        println!("{}", grid.iter().flatten().filter(|c| **c == '#').count());
    }
}
