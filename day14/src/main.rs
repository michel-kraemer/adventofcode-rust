use std::{fs, collections::HashMap};

fn transpose(pattern: Vec<Vec<char>>) -> Vec<Vec<char>> {
    let mut new_pattern = vec![vec![' '; pattern.len()]; pattern[0].len()];
    for (y, row) in pattern.into_iter().enumerate() {
        for (x, cell) in row.iter().enumerate() {
            new_pattern[x][y] = *cell;
        }
    }
    new_pattern
}

fn roll(grid: Vec<Vec<char>>, up: bool) -> Vec<Vec<char>> {
    grid.iter().map(|col| {
        let mut new_col = vec!['.'; col.len()];
        let mut ni = 0;
        for i in 0..col.len() {
            let cell = col[if up { i } else { col.len() - 1 - i }];
            if cell == '#' {
                new_col[if up { i } else { col.len() - 1 - i }] = '#';
                ni = i + 1;
            } else if cell == 'O' {
                new_col[if up { ni } else { col.len() - 1 - ni }] = 'O';
                ni += 1;
            }
        }
        new_col
    }).collect::<Vec<_>>()
}

fn cycle(grid: Vec<Vec<char>>) -> Vec<Vec<char>> {
    // north
    let grid = transpose(grid);
    let grid = roll(grid, true);

    // west
    let grid = transpose(grid);
    let grid = roll(grid, true);

    // south
    let grid = transpose(grid);
    let grid = roll(grid, false);

    // east
    let grid = transpose(grid);
    let grid = roll(grid, false);

    grid
}

fn main() {
    for part1 in [true, false] {
        let input = fs::read_to_string("input.txt").expect("Could not read file");
        let mut grid = input.lines()
            .map(|line| line.chars().collect::<Vec<_>>())
            .collect::<Vec<_>>();

        if part1 {
            grid = transpose(grid);
            grid = roll(grid, true);
        } else {
            let mut seen = HashMap::new();
            let mut steps_left_after_cycle = None;
            for i in 0..1000000000 {
                if steps_left_after_cycle == None {
                    if seen.contains_key(&grid) {
                        let cycle_len = i - seen.get(&grid).unwrap();
                        steps_left_after_cycle = Some((1000000000 - i) % cycle_len);
                    } else {
                        seen.insert(grid.clone(), i);
                    }
                } else {
                    steps_left_after_cycle = Some(steps_left_after_cycle.unwrap() - 1);
                    if let Some(0) = steps_left_after_cycle {
                        break;
                    }
                }
                grid = cycle(grid);
            }
            grid = transpose(grid);
        }

        let mut sum = 0;
        for col in grid {
            for (i, cell) in col.iter().enumerate() {
                if *cell == 'O' {
                    sum += col.len() - i;
                }
            }
        }

        println!("{}", sum);
    }
}
