use std::{collections::HashMap, fs};

fn transpose(pattern: &Vec<Vec<char>>) -> Vec<Vec<char>> {
    let mut new_pattern = vec![vec![' '; pattern.len()]; pattern[0].len()];
    for (y, row) in pattern.iter().enumerate() {
        for (x, cell) in row.iter().enumerate() {
            new_pattern[x][y] = *cell;
        }
    }
    new_pattern
}

#[allow(clippy::needless_range_loop)]
fn main() {
    for part1 in [true, false] {
        let input = fs::read_to_string("input.txt").expect("Could not read file");
        let rules = input
            .lines()
            .map(|l| {
                let p = l.split_once(" => ").unwrap();
                (
                    p.0.split('/')
                        .map(|l| l.chars().collect::<Vec<_>>())
                        .collect::<Vec<_>>(),
                    p.1.split('/')
                        .map(|l| l.chars().collect::<Vec<_>>())
                        .collect::<Vec<_>>(),
                )
            })
            .flat_map(|p| {
                let r1 =
                    p.0.iter()
                        .map(|l| l.iter().rev().copied().collect::<Vec<_>>())
                        .collect::<Vec<_>>();
                let r2 = p.0.iter().rev().map(|c| c.to_vec()).collect::<Vec<_>>();
                let r3 = r2
                    .iter()
                    .map(|l| l.iter().rev().copied().collect::<Vec<_>>())
                    .collect::<Vec<_>>();
                let r4 = transpose(&p.0);
                let r5 = transpose(&r1);
                let r6 = transpose(&r2);
                let r7 = transpose(&r3);
                vec![
                    (p.0, p.1.clone()),
                    (r1, p.1.clone()),
                    (r2, p.1.clone()),
                    (r3, p.1.clone()),
                    (r4, p.1.clone()),
                    (r5, p.1.clone()),
                    (r6, p.1.clone()),
                    (r7, p.1),
                ]
            })
            .collect::<HashMap<_, _>>() // remove duplicates
            .into_iter()
            .collect::<Vec<_>>();

        let mut grid = vec![
            vec!['.', '#', '.'],
            vec!['.', '.', '#'],
            vec!['#', '#', '#'],
        ];

        for _ in 0..(if part1 { 5 } else { 18 }) {
            let n = if grid.len() % 2 == 0 { 2 } else { 3 };

            let mut ng = vec![vec!['.'; grid[0].len() / n * (n + 1)]; grid.len() / n * (n + 1)];
            for r in (0..grid.len()).step_by(n) {
                for c in (0..grid.len()).step_by(n) {
                    for rule in &rules {
                        if rule.0.len() != n {
                            continue;
                        }
                        let mut found = true;
                        for y in 0..n {
                            for x in 0..n {
                                if grid[r + y][c + x] != rule.0[y][x] {
                                    found = false;
                                    break;
                                }
                            }
                        }
                        if found {
                            for y in 0..n + 1 {
                                for x in 0..n + 1 {
                                    ng[r / n * (n + 1) + y][c / n * (n + 1) + x] = rule.1[y][x];
                                }
                            }
                            break;
                        }
                    }
                }
            }
            grid = ng;
        }

        let mut pixels = 0;
        for y in 0..grid.len() {
            for x in 0..grid[y].len() {
                if grid[y][x] == '#' {
                    pixels += 1;
                }
            }
        }

        println!("{}", pixels);
    }
}
