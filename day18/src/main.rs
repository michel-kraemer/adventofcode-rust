mod naive_implementation;

use std::{collections::HashMap, fs};

fn main() {
    for part1 in [true, false] {
        let input = fs::read_to_string("input.txt").expect("Could not read file");
        let lines: Vec<Vec<&str>> = input
            .lines()
            .map(|line| line.split(" ").collect())
            .collect();

        // current drawing position
        let mut x = 0i64;
        let mut y = 0i64;

        // recorded edge points: y -> (x, y, type)
        let mut grid: HashMap<i64, Vec<(i64, i64, char)>> = HashMap::new();

        // In my input file this was correct but other input files might need
        // other values (one of R, L, U, D). This should be the value of
        // `prev_dir` at the end of the drawn polygon (at the end of the for
        // loop below).
        let mut prev_dir = "U";

        for line in &lines {
            // decode next direction and distance
            let (dir, dist) = if part1 {
                (line[0], line[1].parse::<i64>().unwrap())
            } else {
                let hex = line[2];
                (
                    match hex.chars().nth(7).unwrap() {
                        '0' => "R",
                        '1' => "D",
                        '2' => "L",
                        '3' => "U",
                        _ => panic!(),
                    },
                    i64::from_str_radix(&hex[2..7], 16).unwrap(),
                )
            };

            // turn into the given direction and record the correct corner type
            // (see day 10 for the meaning of L, F, J, 7)
            match dir {
                "R" => {
                    match prev_dir {
                        "D" => grid.entry(y).or_default().push((x, y, 'L')),
                        "U" => grid.entry(y).or_default().push((x, y, 'F')),
                        _ => panic!(),
                    }
                    x += dist;
                }
                "L" => {
                    match prev_dir {
                        "D" => grid.entry(y).or_default().push((x, y, 'J')),
                        "U" => grid.entry(y).or_default().push((x, y, '7')),
                        _ => panic!(),
                    }
                    x -= dist;
                }
                "D" => {
                    match prev_dir {
                        "R" => grid.entry(y).or_default().push((x, y, '7')),
                        "L" => grid.entry(y).or_default().push((x, y, 'F')),
                        _ => panic!(),
                    }
                    y += dist;
                }
                "U" => {
                    match prev_dir {
                        "R" => grid.entry(y).or_default().push((x, y, 'J')),
                        "L" => grid.entry(y).or_default().push((x, y, 'L')),
                        _ => panic!(),
                    }
                    y -= dist;
                }
                _ => {}
            }
            prev_dir = dir;
        }

        // convert hashmap to grid and sort it by y-value
        let mut grid = grid.into_iter().collect::<Vec<_>>();
        grid.sort_by(|a, b| a.0.cmp(&b.0));

        // insert blank rows into grid where there is a gap between the
        // y-values of two subsequent rows
        let mut new_rows: Vec<(i64, Vec<(i64, i64, char)>)> = Vec::new();
        for (i, row) in grid.into_iter().enumerate() {
            if i > 0 && row.0 > new_rows[new_rows.len() - 1].0 + 1 {
                new_rows.push((-1, Vec::new()));
            }
            new_rows.push(row);
        }
        let mut grid = new_rows;

        // find all downward pointing corners (F or 7) in the grid and insert
        // vertical connections ('|') into all subsequent lines until an
        // upward pointing corner (J or L) is reached
        for i in 0..grid.len() {
            let (current_row, next_rows) = grid.split_at_mut(i + 1);
            let row = &current_row[i];
            for j in 0..row.1.len() {
                if row.1[j].2 == 'F' || row.1[j].2 == '7' {
                    for next_row in next_rows.iter_mut() {
                        if !next_row
                            .1
                            .iter()
                            .any(|c| c.0 == row.1[j].0 && (c.2 == 'J' || c.2 == 'L'))
                        {
                            next_row.1.push((row.1[j].0, next_row.0, '|'));
                        } else {
                            break;
                        }
                    }
                }
            }
        }

        // sort cells in all rows by x-value
        let mut new_rows = Vec::new();
        for mut r in grid {
            r.1.sort_by(|a, b| a.0.cmp(&b.0));
            new_rows.push((r.0, r.1));
        }
        let grid = new_rows;

        // do basically the same as on day 10 but handle gaps in the grid correctly
        let mut area = 0;
        for i in 0..grid.len() {
            // true if inside the polygon
            let mut inside = false;

            // current row to analyze
            let row = &grid[i].1;

            // index in current row
            let mut j = 0;

            // index where a left border of the polygon was encountered
            let mut last_left_border = 0;

            // area of this row
            let mut row_area = 0;

            while j < row.len() {
                match row[j].2 {
                    '|' => {
                        if !inside {
                            // found left border
                            last_left_border = row[j].0;
                            inside = true;
                        } else {
                            // found right border - add area between here
                            // and last left border
                            row_area += row[j].0 - last_left_border + 1;
                            inside = false;
                        }
                    }

                    'F' => {
                        let last_inside = inside;

                        // get length of this edge
                        let s = row[j].0;
                        j += 1;
                        let e = row[j].0;

                        // if end of edge points upwards, we found a
                        // vertical border
                        if row[j].2 == 'J' {
                            row_area += e - s + 1;
                            inside = !inside;
                        } else if !inside {
                            // only add to row area if we are currently not
                            // inside the polygon and the edge is not a
                            // vertical border
                            row_area += e - s + 1;
                        }

                        if last_inside && !inside {
                            row_area += row[j - 1].0 - last_left_border;
                        } else if !last_inside && inside {
                            last_left_border = row[j].0 + 1;
                        }
                    }

                    'L' => {
                        // similar to F case above ...
                        let last_inside = inside;

                        let s = row[j].0;
                        j += 1;
                        let e = row[j].0;
                        
                        if row[j].2 == '7' {
                            row_area += e - s + 1;
                            inside = !inside;
                        } else if !inside {
                            row_area += e - s + 1;
                        }
                        
                        if last_inside && !inside {
                            row_area += row[j - 1].0 - last_left_border;
                        } else if !last_inside && inside {
                            last_left_border = row[j].0 + 1;
                        }
                    }

                    _ => {}
                }
                j += 1;
            }

            if grid[i].0 == -1 {
                // multiply row_area if the current row is one of the blank
                // lines we filled in earlier
                area += row_area * (grid[i + 1].0 - grid[i - 1].0 - 1);
            } else {
                area += row_area;
            }
        }

        println!("{area}");
    }
}
