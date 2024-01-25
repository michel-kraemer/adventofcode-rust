use std::{cmp::min, collections::HashMap, fs, rc::Rc};

#[allow(clippy::needless_range_loop)]
fn main() {
    for part1 in [true, false] {
        let input = fs::read_to_string("input.txt").expect("Could not read file");
        let mut grid = Rc::new(
            input
                .lines()
                .map(|l| l.chars().collect::<Vec<_>>())
                .collect::<Vec<_>>(),
        );

        let mut seen = HashMap::new();
        seen.insert(grid.clone(), 0);

        let max_steps = if part1 { 10 } else { 1_000_000_000 };
        let mut s = 0;
        while s < max_steps {
            let mut ng = (*grid).clone();

            for y in 0..grid.len() {
                for x in 0..grid[y].len() {
                    match grid[y][x] {
                        '.' => {
                            let mut count = 0;
                            for y1 in y.saturating_sub(1)..=min(y + 1, grid.len() - 1) {
                                for x1 in x.saturating_sub(1)..=min(x + 1, grid[y1].len() - 1) {
                                    if (y1, x1) == (y, x) {
                                        continue;
                                    }
                                    if grid[y1][x1] == '|' {
                                        count += 1;
                                    }
                                }
                            }
                            if count >= 3 {
                                ng[y][x] = '|';
                            }
                        }

                        '|' => {
                            let mut count = 0;
                            for y1 in y.saturating_sub(1)..=min(y + 1, grid.len() - 1) {
                                for x1 in x.saturating_sub(1)..=min(x + 1, grid[y1].len() - 1) {
                                    if (y1, x1) == (y, x) {
                                        continue;
                                    }
                                    if grid[y1][x1] == '#' {
                                        count += 1;
                                    }
                                }
                            }
                            if count >= 3 {
                                ng[y][x] = '#';
                            }
                        }

                        '#' => {
                            let mut count_lumb = 0;
                            let mut count_trees = 0;
                            for y1 in y.saturating_sub(1)..=min(y + 1, grid.len() - 1) {
                                for x1 in x.saturating_sub(1)..=min(x + 1, grid[y1].len() - 1) {
                                    if (y1, x1) == (y, x) {
                                        continue;
                                    }
                                    if grid[y1][x1] == '|' {
                                        count_trees += 1;
                                    }
                                    if grid[y1][x1] == '#' {
                                        count_lumb += 1;
                                    }
                                }
                            }
                            if count_trees == 0 || count_lumb == 0 {
                                ng[y][x] = '.';
                            }
                        }

                        _ => panic!(),
                    }
                }
            }

            s += 1;
            grid = Rc::new(ng);

            if let Some(start) = seen.get(&grid) {
                s = max_steps - ((max_steps - s) % (s - start));
                seen.clear();
            } else {
                seen.insert(grid.clone(), s);
            }
        }

        let mut sum_wood = 0;
        let mut sum_lumb = 0;
        for y in 0..grid.len() {
            for x in 0..grid[y].len() {
                match grid[y][x] {
                    '|' => sum_wood += 1,
                    '#' => sum_lumb += 1,
                    _ => {}
                }
            }
        }

        println!("{}", sum_lumb * sum_wood);
    }
}
