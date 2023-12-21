use std::{fs, collections::{HashSet, HashMap}};

fn main() {
    let input = fs::read_to_string("input.txt").expect("Could not read file");
    let grid = input.lines().map(|l| l.chars().collect::<Vec<_>>()).collect::<Vec<_>>();

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

    let mut distances: HashMap<(usize, usize), usize> = HashMap::new();
    let mut marked_cells: HashSet<(usize, usize)> = HashSet::new();
    marked_cells.insert(start);

    let w = grid[0].len();
    let h = grid.len();
    for i in 1..=max_steps {
        let mut new_pos = HashSet::new();
        for mc in &marked_cells {
            if mc.0 > 0 && grid[mc.1][mc.0 - 1] != '#' {
                let p = (mc.0 - 1, mc.1);
                if !distances.contains_key(&p) {
                    distances.insert(p, i);
                    new_pos.insert(p);
                }
            }
            if mc.1 > 0 && grid[mc.1 - 1][mc.0] != '#' {
                let p = (mc.0, mc.1 - 1);
                if !distances.contains_key(&p) {
                    distances.insert(p, i);
                    new_pos.insert(p);
                }
            }
            if mc.1 < w - 1 && grid[mc.1][mc.0 + 1] != '#' {
                let p = (mc.0 + 1, mc.1);
                if !distances.contains_key(&p) {
                    distances.insert(p, i);
                    new_pos.insert(p);
                }
            }
            if mc.1 < h - 1 && grid[mc.1 + 1][mc.0] != '#' {
                let p = (mc.0, mc.1 + 1);
                if !distances.contains_key(&p) {
                    distances.insert(p, i);
                    new_pos.insert(p);
                }
            }
        }

        marked_cells = new_pos;
    }

    let filtered_distances = distances.into_iter().filter(|f| f.1 % 2 == 0).collect::<Vec<_>>();

    println!("{}", filtered_distances.len());
}
