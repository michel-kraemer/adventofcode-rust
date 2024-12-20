use std::fs;

const DIRS: [(i32, i32); 4] = [(1, 0), (0, 1), (-1, 0), (0, -1)];

fn find_non_branching_path(
    grid: &[u8],
    width: usize,
    start: (usize, usize),
    end: (usize, usize),
) -> Vec<(usize, usize)> {
    // find start direction
    let mut dir = 0;
    for (i, d) in DIRS.iter().enumerate() {
        let nx = start.0 as i32 + d.0;
        let ny = start.1 as i32 + d.1;
        if grid[ny as usize * width + nx as usize] != b'#' {
            dir = i;
            break;
        }
    }

    // follow path and only turn right or left until we reach the end
    let mut pos = start;
    let mut result = vec![start];
    while pos != end {
        let nx = pos.0 as i32 + DIRS[dir].0;
        let ny = pos.1 as i32 + DIRS[dir].1;

        if grid[ny as usize * width + nx as usize] == b'#' {
            // can we turn right?
            let right = (dir + 1) % 4;
            if grid[(pos.1 as i32 + DIRS[right].1) as usize * width
                + (pos.0 as i32 + DIRS[right].0) as usize]
                != b'#'
            {
                dir = right;
                continue;
            }

            // can we turn left?
            let left = (dir + 3) % 4;
            if grid[(pos.1 as i32 + DIRS[left].1) as usize * width
                + (pos.0 as i32 + DIRS[left].0) as usize]
                != b'#'
            {
                dir = left;
                continue;
            }

            // we're in a dead end
            unreachable!("Path must not branch");
        }

        pos = (nx as usize, ny as usize);
        result.push(pos);
    }

    result
}

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

    let path = find_non_branching_path(&grid, width, start, end);
    let max = path.len() - 1;

    let mut total1 = 0;
    let mut total2 = 0;
    for i in 0..path.len() {
        for j in i + 101..path.len() {
            let s = path[i];
            let e = path[j];
            let dist = s.0.abs_diff(e.0) + s.1.abs_diff(e.1);
            let m = max - (j - i) + dist;
            if m >= max || max - m < 100 {
                continue;
            }

            if dist <= 2 {
                total1 += 1;
            }
            if dist <= 20 {
                total2 += 1;
            }
        }
    }

    println!("{}", total1);
    println!("{}", total2);
}
