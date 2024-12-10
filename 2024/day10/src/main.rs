use std::collections::{HashSet, VecDeque};
use std::fs;

fn main() {
    let input = fs::read_to_string("input.txt").expect("Could not read file");
    let lines = input.lines().collect::<Vec<_>>();
    let width = lines[0].len();
    let height = lines.len();
    let grid = lines
        .iter()
        .flat_map(|l| l.as_bytes())
        .copied()
        .collect::<Vec<_>>();

    let mut trailheads = Vec::new();
    for y in 0..height {
        for x in 0..width {
            if grid[y * width + x] == b'0' {
                trailheads.push((x as i32, y as i32, b'0'));
            }
        }
    }

    let mut total1 = 0;
    let mut total2 = 0;
    for t in trailheads {
        let mut queue = VecDeque::new();
        queue.push_back(t);
        let mut unique = HashSet::new();
        let mut rating = 0;
        while let Some(pos) = queue.pop_front() {
            if pos.2 == b'9' {
                unique.insert((pos.0, pos.1));
                rating += 1;
                continue;
            }
            for d in [(1, 0), (-1, 0), (0, 1), (0, -1)] {
                let nx = pos.0 + d.0;
                let ny = pos.1 + d.1;
                if nx >= 0 && ny >= 0 && nx < width as i32 && ny < height as i32 {
                    let d = grid[ny as usize * width + nx as usize];
                    if d == pos.2 + 1 {
                        queue.push_back((nx, ny, d));
                    }
                }
            }
        }
        total1 += unique.len();
        total2 += rating;
    }

    println!("{}", total1);
    println!("{}", total2);
}
