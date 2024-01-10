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

fn hash(p: &[Vec<char>], x: usize, y: usize, n: usize) -> u32 {
    let mut r = if n == 3 { 0 } else { 1 << 16 };
    let mut i = 0;
    for row in 0..n {
        for col in 0..n {
            if p[row + y][col + x] == '#' {
                r |= 1 << i;
            }
            i += 1;
        }
    }
    r
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
                let p2 =
                    p.0.iter()
                        .map(|l| l.iter().rev().copied().collect::<Vec<_>>())
                        .collect::<Vec<_>>();
                let p3 = p.0.iter().rev().map(|c| c.to_vec()).collect::<Vec<_>>();
                let p4 = p3
                    .iter()
                    .map(|l| l.iter().rev().copied().collect::<Vec<_>>())
                    .collect::<Vec<_>>();
                let p5 = transpose(&p.0);
                let p6 = transpose(&p2);
                let p7 = transpose(&p3);
                let p8 = transpose(&p4);

                let h1 = hash(&p.0, 0, 0, p.0.len());
                let h2 = hash(&p2, 0, 0, p2.len());
                let h3 = hash(&p3, 0, 0, p3.len());
                let h4 = hash(&p4, 0, 0, p4.len());
                let h5 = hash(&p5, 0, 0, p5.len());
                let h6 = hash(&p6, 0, 0, p6.len());
                let h7 = hash(&p7, 0, 0, p7.len());
                let h8 = hash(&p8, 0, 0, p8.len());

                vec![
                    (h1, p.1.clone()),
                    (h2, p.1.clone()),
                    (h3, p.1.clone()),
                    (h4, p.1.clone()),
                    (h5, p.1.clone()),
                    (h6, p.1.clone()),
                    (h7, p.1.clone()),
                    (h8, p.1),
                ]
            })
            .collect::<HashMap<_, _>>();

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
                    let h = hash(&grid, c, r, n);
                    let rule = &rules[&h];
                    for y in 0..n + 1 {
                        for x in 0..n + 1 {
                            ng[r / n * (n + 1) + y][c / n * (n + 1) + x] = rule[y][x];
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
