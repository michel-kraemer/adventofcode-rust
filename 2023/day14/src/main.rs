use std::{collections::HashMap, fs};

fn rotate(grid: &[Vec<char>]) -> Vec<Vec<char>> {
    let mut new_grid = vec![vec![' '; grid.len()]; grid[0].len()];
    for y in 0..new_grid.len() {
        for x in 0..new_grid[0].len() {
            new_grid[y][grid.len() - 1 - x] = grid[x][y];
        }
    }
    new_grid
}

fn roll_to_north(grid: &mut [Vec<char>]) {
    for y in 0..grid.len() {
        for x in 0..grid[y].len() {
            if grid[y][x] == 'O' {
                grid[y][x] = '.';
                let mut y2 = y;
                while y2 > 0 && grid[y2 - 1][x] == '.' {
                    y2 -= 1;
                }
                grid[y2][x] = 'O';
            }
        }
    }
}

fn calculate_load(grid: &[Vec<char>]) -> usize {
    let mut total = 0;
    for y in 0..grid.len() {
        for x in 0..grid[y].len() {
            if grid[y][x] == 'O' {
                total += grid.len() - y;
            }
        }
    }
    total
}

fn main() {
    let input = fs::read_to_string("input.txt").expect("Could not read file");
    let mut grid = input
        .lines()
        .map(|l| l.chars().collect::<Vec<_>>())
        .collect::<Vec<_>>();

    // part 1
    let mut part1_clone = grid.clone();
    roll_to_north(&mut part1_clone);
    println!("{}", calculate_load(&part1_clone));

    // part 2
    let max_steps = 1000000000;
    let mut seen = HashMap::new();
    let mut step = 0;
    while step < max_steps {
        for _ in 0..4 {
            roll_to_north(&mut grid);
            grid = rotate(&grid);
        }
        step += 1;

        if let Some(i) = seen.get(&grid) {
            step = max_steps - (max_steps - i) % (step - i);
            seen.clear();
        } else {
            seen.insert(grid.clone(), step);
        }
    }

    println!("{}", calculate_load(&grid));
}
