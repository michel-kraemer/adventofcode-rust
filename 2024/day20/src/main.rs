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

    // find path from start to end
    let path = find_non_branching_path(&grid, width, start, end);
    let max = path.len() - 1;

    // Insert waypoints into an index. This allows us to limit the number of
    // neighbors we need to check later.
    let cell_size = 20;
    let cells_x = (width + cell_size - 1) / cell_size;
    let cells_y = (height + cell_size - 1) / cell_size;
    let mut cells = vec![vec![]; cells_x * cells_y];
    for (i, &(px, py)) in path.iter().enumerate() {
        cells[(py / cell_size) * cells_x + (px / cell_size)].push((i, px, py));
    }

    // For each pair of waypoint and neighbor, check if their distance is less
    // than or equal to 2 (part 1) or 20 (part 2). If so, check if shortcutting
    // from the waypoint to the neighbor would actually make the path shorter.
    let mut total1 = 0;
    let mut total2 = 0;
    for cy in 0..cells_y {
        for cx in 0..cells_x {
            let min_cx = cx.saturating_sub(1);
            let max_cx = (cells_x - 1).min(cx + 1);
            let min_cy = cy.saturating_sub(1);
            let max_cy = (cells_y - 1).min(cy + 1);

            while !cells[cy * cells_x + cx].is_empty() {
                let (si, sx, sy) = cells[cy * cells_x + cx].swap_remove(0);

                for cy2 in min_cy..=max_cy {
                    for cx2 in min_cx..=max_cx {
                        for &(ei, ex, ey) in &cells[cy2 * cells_x + cx2] {
                            let dist = sx.abs_diff(ex) + sy.abs_diff(ey);
                            let m = max - ei.abs_diff(si) + dist;
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
                }
            }
        }
    }

    println!("{}", total1);
    println!("{}", total2);
}
