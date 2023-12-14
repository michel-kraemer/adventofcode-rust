use std::fs;

fn transpose(grid: Vec<Vec<char>>) -> Vec<Vec<char>> {
    let mut new_grid = vec![vec![' '; grid.len()]; grid[0].len()];
    for (y, row) in grid.into_iter().enumerate() {
        for (x, cell) in row.iter().enumerate() {
            new_grid[x][y] = *cell;
        }
    }
    new_grid
}

fn find_empty(grid: &Vec<Vec<char>>) -> Vec<usize> {
    let mut result = Vec::new();
    for (y, row) in grid.iter().enumerate() {
        if row.iter().all(|c| *c == '.') {
            result.push(y);
        }
    }
    result
}

fn main() {
    for part1 in [true, false] {
        let factor = if part1 { 2 } else { 1000000 };

        let input = fs::read_to_string("input.txt").expect("Could not read file");
        let grid = input.lines().map(|l| l.chars().collect::<Vec<_>>()).collect::<Vec<_>>();

        // expand empty rows
        let empty_rows = find_empty(&grid);

        // expand empty columns
        let grid = transpose(grid);
        let empty_cols = find_empty(&grid);
        let grid = transpose(grid);

        // find coordinates of galaxies
        let mut galaxies = Vec::new();
        for (y, row) in grid.iter().enumerate() {
            for (x, cell) in row.iter().enumerate() {
                if *cell == '#' {
                    galaxies.push((x, y));
                }
            }
        }

        let mut sum = 0;
        for i in 0..galaxies.len() - 1 {
            for j in i + 1..galaxies.len() {
                let (x0, x1) = if galaxies[i].0 < galaxies[j].0 {
                    (galaxies[i].0, galaxies[j].0)
                } else {
                    (galaxies[j].0, galaxies[i].0)
                };
                let (y0, y1) = if galaxies[i].1 < galaxies[j].1 {
                    (galaxies[i].1, galaxies[j].1)
                } else {
                    (galaxies[j].1, galaxies[i].1)
                };
                let mut dx = x1 - x0;
                let mut dy = y1 - y0;
                for x in &empty_cols {
                    if x0 <= *x && *x <= x1 {
                        dx += factor - 1;
                    }
                }
                for y in &empty_rows {
                    if y0 <= *y && *y <= y1 {
                        dy += factor - 1;
                    }
                }
                let dist =  dx + dy;
                sum += dist;
            }
        }

        println!("{}", sum);
    }
}
