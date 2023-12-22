use std::{collections::VecDeque, fs};

fn flood(grid: &Vec<Vec<char>>, start: &(usize, usize)) -> Vec<Vec<usize>> {
    let w = grid[0].len();
    let h = grid.len();

    let mut distances = vec![vec![0; w]; h];
    let mut marked_cells = VecDeque::new();
    marked_cells.push_back((start.0, start.1, 0usize));

    while let Some(mc) = marked_cells.pop_front() {
        if mc.0 > 0 && grid[mc.1][mc.0 - 1] != '#' {
            let p = (mc.0 - 1, mc.1, mc.2 + 1);
            if distances[p.1][p.0] == 0 {
                distances[p.1][p.0] = p.2;
                marked_cells.push_back(p);
            }
        }
        if mc.1 > 0 && grid[mc.1 - 1][mc.0] != '#' {
            let p = (mc.0, mc.1 - 1, mc.2 + 1);
            if distances[p.1][p.0] == 0 {
                distances[p.1][p.0] = p.2;
                marked_cells.push_back(p);
            }
        }
        if mc.0 < w - 1 && grid[mc.1][mc.0 + 1] != '#' {
            let p = (mc.0 + 1, mc.1, mc.2 + 1);
            if distances[p.1][p.0] == 0 {
                distances[p.1][p.0] = p.2;
                marked_cells.push_back(p);
            }
        }
        if mc.1 < h - 1 && grid[mc.1 + 1][mc.0] != '#' {
            let p = (mc.0, mc.1 + 1, mc.2 + 1);
            if distances[p.1][p.0] == 0 {
                distances[p.1][p.0] = p.2;
                marked_cells.push_back(p);
            }
        }
    }

    distances
}

fn main() {
    let input = fs::read_to_string("input.txt").expect("Could not read file");
    let grid = input
        .lines()
        .map(|l| l.chars().collect::<Vec<_>>())
        .collect::<Vec<_>>();

    let max_steps = 64usize;

    let mut start = (0usize, 0usize);
    for (y, row) in grid.iter().enumerate() {
        for (x, cell) in row.iter().enumerate() {
            if *cell == 'S' {
                start = (x, y);
                break;
            }
        }
    }

    let distances = flood(&grid, &start);
    let mut sum = 0;
    for y in 0..distances.len() {
        for x in 0..distances[y].len() {
            let d = distances[y][x];
            if d != 0 && d <= max_steps && d % 2 == 0 {
                sum += 1;
            }
        }
    }

    println!("{}", sum);
}
