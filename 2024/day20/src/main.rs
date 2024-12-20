use std::{
    collections::{HashSet, VecDeque},
    fs,
};

fn bfs(
    grid: &[u8],
    width: usize,
    height: usize,
    start: (usize, usize),
    end: (usize, usize),
) -> Vec<(usize, usize)> {
    let mut seen = HashSet::new();

    let mut queue = VecDeque::new();
    queue.push_back((start.0, start.1, vec![start]));

    while let Some((x, y, path)) = queue.pop_front() {
        if seen.contains(&(x, y)) {
            continue;
        }
        seen.insert((x, y));

        if x == end.0 && y == end.1 {
            return path;
        }

        for (dx, dy) in [(1, 0), (0, 1), (-1, 0), (0, -1)] {
            let nx = x as i32 + dx;
            let ny = y as i32 + dy;
            if nx >= 0
                && ny >= 0
                && nx < width as i32
                && ny < height as i32
                && grid[ny as usize * width + nx as usize] != b'#'
            {
                let mut new_path = path.clone();
                new_path.push((nx as usize, ny as usize));
                queue.push_back((nx as usize, ny as usize, new_path));
            }
        }
    }

    unreachable!()
}

fn main() {
    for part1 in [true, false] {
        let input = fs::read_to_string("input.txt").expect("Could not read file");
        let lines = input.lines().collect::<Vec<_>>();
        let width = lines[0].len();
        let height = lines.len();
        let grid = lines
            .iter()
            .flat_map(|l| l.as_bytes())
            .copied()
            .collect::<Vec<_>>();

        let mut start = (0, 0);
        let mut end = (0, 0);
        for y in 0..height {
            for x in 0..width {
                let c = grid[y * width + x];
                if c == b'S' {
                    start = (x, y);
                } else if c == b'E' {
                    end = (x, y);
                }
            }
        }

        let path = bfs(&grid, width, height, start, end);
        let max = path.len() - 1;
        let max_cheat_len = if part1 { 2 } else { 20 };

        let mut total = 0;
        for i in 0..path.len() {
            for j in i + 1..path.len() {
                let s = path[i];
                let e = path[j];
                let dist = (s.0.abs_diff(e.0) + s.1.abs_diff(e.1)) as usize;
                if dist > max_cheat_len {
                    continue;
                }

                let m = path.len() - (j - i + 1) + dist;
                if m < max && max - m >= 100 {
                    total += 1;
                }
            }
        }

        println!("{}", total);
    }
}
