use std::fs;

fn transpose(pattern: Vec<Vec<char>>) -> Vec<Vec<char>> {
    let mut new_pattern = vec![vec![' '; pattern.len()]; pattern[0].len()];
    for (y, row) in pattern.into_iter().enumerate() {
        for (x, cell) in row.iter().enumerate() {
            new_pattern[x][y] = *cell;
        }
    }
    new_pattern
}

fn roll_north(grid: Vec<Vec<char>>) -> Vec<Vec<char>> {
    grid.iter().map(|col| {
        let mut new_col = vec!['.'; col.len()];
        let mut ni = 0;
        for i in 0..col.len() {
            let cell = col[i];
            if cell == '#' {
                new_col[i] = '#';
                ni = i + 1;
            } else if cell == 'O' {
                new_col[ni] = 'O';
                ni += 1;
            }
        }
        new_col
    }).collect::<Vec<_>>()
}

fn roll_south(grid: Vec<Vec<char>>) -> Vec<Vec<char>> {
    grid.iter().map(|col| {
        let mut new_col = vec!['.'; col.len()];
        let mut ni = col.len() - 1;
        for i in (0..col.len()).rev() {
            let cell = col[i];
            if cell == '#' {
                new_col[i] = '#';
                ni = i - 1;
            } else if cell == 'O' {
                new_col[ni] = 'O';
                ni -= 1;
            }
        }
        new_col
    }).collect::<Vec<_>>()
}

fn cycle(grid: Vec<Vec<char>>) -> Vec<Vec<char>> {
    // north
    let grid = transpose(grid);
    let grid = roll_north(grid);

    // west
    let grid = transpose(grid);
    let grid = roll_north(grid);

    // south
    let grid = transpose(grid);
    let grid = roll_south(grid);

    // east
    let grid = transpose(grid);
    let grid = roll_south(grid);

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
            grid = roll_north(grid);
        } else {
            // exactly only 1000 iterations are necessary - after that, the pattern repeats
            for _ in 0..1000 {
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
