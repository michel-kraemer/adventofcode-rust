// While I was able to solve the first part on my own, I needed help with the
// second part. All the credits for the second part therefore go to
// HyperNeutrino (https://www.youtube.com/@hyper-neutrino /
// https://github.com/hyper-neutrino/advent-of-code/) and especially this video:
// https://www.youtube.com/watch?v=9UOMZSL0JTg

use std::{collections::VecDeque, fs};

fn flood(grid: &Vec<Vec<char>>, start: &(usize, usize)) -> Vec<Vec<usize>> {
    let w = grid[0].len();
    let h = grid.len();

    let mut distances = vec![vec![0; w]; h];
    let mut marked_cells = VecDeque::new();
    marked_cells.push_back((start.0, start.1, 0));

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

fn count(distances: &Vec<Vec<usize>>, max_steps: usize) -> usize {
    let even = max_steps % 2 == 0;
    let mut sum = 0;
    for y in 0..distances.len() {
        for x in 0..distances[y].len() {
            let d = distances[y][x];
            if distances[y][x] != 0 && d <= max_steps && d % 2 == (if even { 0 } else { 1 }) {
                sum += 1;
            }
        }
    }
    sum
}

fn main() {
    for part1 in [true, false] {
        let max_steps = if part1 { 64usize } else { 26501365usize };

        let input = fs::read_to_string("input.txt").expect("Could not read file");
        let grid = input
            .lines()
            .map(|l| l.chars().collect::<Vec<_>>())
            .collect::<Vec<_>>();

        let mut start = (0usize, 0usize);
        for (y, row) in grid.iter().enumerate() {
            for (x, cell) in row.iter().enumerate() {
                if *cell == 'S' {
                    start = (x, y);
                    break;
                }
            }
        }

        let sum = if part1 {
            let distances = flood(&grid, &start);
            count(&distances, max_steps)
        } else {
            let c_odd = count(&flood(&grid, &start), grid.len() * 2 + 1);
            let c_even = count(&flood(&grid, &start), grid.len() * 2);

            let ct = count(&flood(&grid, &(start.0, grid.len() - 1)), grid.len() - 1);
            let cr = count(&flood(&grid, &(0, start.1)), grid.len() - 1);
            let cl = count(&flood(&grid, &(grid.len() - 1, start.1)), grid.len() - 1);
            let cb = count(&flood(&grid, &(start.0, 0)), grid.len() - 1);

            let c_tr_s = count(&flood(&grid, &(0, grid.len() - 1)), grid.len() / 2 - 1);
            let c_br_s = count(&flood(&grid, &(0, 0)), grid.len() / 2 - 1);
            let c_tl_s = count(
                &flood(&grid, &(grid.len() - 1, grid.len() - 1)),
                grid.len() / 2 - 1,
            );
            let c_bl_s = count(&flood(&grid, &(grid.len() - 1, 0)), grid.len() / 2 - 1);

            let c_tr_l = count(&flood(&grid, &(0, grid.len() - 1)), grid.len() * 3 / 2 - 1);
            let c_br_l = count(&flood(&grid, &(0, 0)), grid.len() * 3 / 2 - 1);
            let c_tl_l = count(
                &flood(&grid, &(grid.len() - 1, grid.len() - 1)),
                grid.len() * 3 / 2 - 1,
            );
            let c_bl_l = count(&flood(&grid, &(grid.len() - 1, 0)), grid.len() * 3 / 2 - 1);

            let nadd = max_steps / grid.len() - 1;
            let odd = (nadd / 2 * 2 + 1).pow(2);
            let even = ((nadd + 1) / 2 * 2).pow(2);
            c_odd * odd
                + c_even * even
                + ct
                + cr
                + cl
                + cb
                + (nadd + 1) * (c_tr_s + c_br_s + c_tl_s + c_bl_s)
                + nadd * (c_tr_l + c_br_l + c_tl_l + c_bl_l)
        };

        println!("{}", sum);
    }
}
