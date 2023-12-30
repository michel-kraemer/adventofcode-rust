use std::fs;

fn transpose(pattern: Vec<Vec<bool>>) -> Vec<Vec<bool>> {
    let mut new_pattern = vec![vec![false; pattern.len()]; pattern[0].len()];
    for (y, row) in pattern.into_iter().enumerate() {
        for (x, cell) in row.iter().enumerate() {
            new_pattern[x][y] = *cell;
        }
    }
    new_pattern
}

#[allow(clippy::needless_range_loop)]
fn main() {
    let input = fs::read_to_string("input.txt").expect("Could not read file");
    let instructions = input.lines().collect::<Vec<_>>();

    let mut grid = vec![vec![false; 50]; 6];
    for i in instructions {
        if i.starts_with("rect") {
            let (_, size) = i.split_once(' ').unwrap();
            let (w, h) = size.split_once('x').unwrap();
            let w = w.parse::<usize>().unwrap();
            let h = h.parse::<usize>().unwrap();
            for x in 0..w {
                for y in 0..h {
                    grid[y][x] = true;
                }
            }
        } else if i.starts_with("rotate row") {
            let remainder = &i[13..];
            let (row, len) = remainder.split_once(" by ").unwrap();
            let row = row.parse::<usize>().unwrap();
            let len = len.parse::<usize>().unwrap();
            grid[row].rotate_right(len);
        } else if i.starts_with("rotate column") {
            let remainder = &i[16..];
            let (col, len) = remainder.split_once(" by ").unwrap();
            let col = col.parse::<usize>().unwrap();
            let len = len.parse::<usize>().unwrap();
            grid = transpose(grid);
            grid[col].rotate_right(len);
            grid = transpose(grid);
        }
    }

    let mut result = 0;
    for y in 0..grid.len() {
        for x in 0..grid[y].len() {
            if grid[y][x] {
                result += 1;
            }
        }
    }

    println!("{}", result);

    grid.iter().for_each(|r| {
        println!(
            "{}",
            String::from_iter(r.iter().map(|b| if *b { '#' } else { '.' }))
        )
    });
}
