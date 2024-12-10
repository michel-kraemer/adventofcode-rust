use std::collections::VecDeque;
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
                trailheads.push((x as i32, y as i32));
            }
        }
    }

    let mut total1 = 0;
    let mut total2 = 0;
    let mut queue = VecDeque::new();
    let mut seen = vec![false; grid.len()];
    for t in trailheads {
        queue.push_back(t);
        seen.fill(false);
        while let Some(pos) = queue.pop_front() {
            let i = pos.1 as usize * width + pos.0 as usize;
            let c = grid[i];
            if c == b'9' {
                if !seen[i] {
                    total1 += 1;
                    seen[i] = true;
                }
                total2 += 1;
                continue;
            }
            for dir in [(1, 0), (-1, 0), (0, 1), (0, -1)] {
                let nx = pos.0 + dir.0;
                let ny = pos.1 + dir.1;
                if nx >= 0 && ny >= 0 && nx < width as i32 && ny < height as i32 {
                    let d = grid[ny as usize * width + nx as usize];
                    if d == c + 1 {
                        queue.push_back((nx, ny));
                    }
                }
            }
        }
    }

    println!("{}", total1);
    println!("{}", total2);
}
