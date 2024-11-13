use std::fs;

fn main() {
    for part1 in [true, false] {
        let additional_steps_per_empty = if part1 { 2 } else { 1000000 };

        let input = fs::read_to_string("input.txt").expect("Could not read file");
        let grid = input
            .lines()
            .map(|l| l.chars().collect::<Vec<_>>())
            .collect::<Vec<_>>();

        // find empty rows
        let mut empty_rows = Vec::new();
        for (y, row) in grid.iter().enumerate() {
            if row.iter().all(|c| *c == '.') {
                empty_rows.push(y);
            }
        }

        // find empty columns
        let mut empty_cols = Vec::new();
        for x in 0..grid[0].len() {
            let mut all_empty = true;
            for row in &grid {
                if row[x] != '.' {
                    all_empty = false;
                    break;
                }
            }
            if all_empty {
                empty_cols.push(x);
            }
        }

        // find galaxies
        let mut galaxies = Vec::new();
        for (y, row) in grid.iter().enumerate() {
            for (x, c) in row.iter().enumerate() {
                if *c == '#' {
                    galaxies.push((x, y));
                }
            }
        }

        // calculate manhattan distances between all pairs and add additional
        // steps for empty rows and columns
        let mut total = 0;
        for (i, g1) in galaxies.iter().enumerate() {
            for g2 in galaxies.iter().skip(i + 1) {
                let col_range = g1.0.min(g2.0)..g1.0.max(g2.0);
                let row_range = g1.1.min(g2.1)..g1.1.max(g2.1);
                let mut steps = col_range.len() + row_range.len();
                for c in &empty_cols {
                    if col_range.contains(c) {
                        steps += additional_steps_per_empty - 1;
                    }
                }
                for r in &empty_rows {
                    if row_range.contains(r) {
                        steps += additional_steps_per_empty - 1;
                    }
                }
                total += steps;
            }
        }

        println!("{}", total);
    }
}
