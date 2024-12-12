use std::collections::VecDeque;
use std::fs;

struct FillGridResult {
    area: usize,
    horizonzal_sides: Vec<(i32, i32)>,
    vertical_sides: Vec<(i32, i32)>,
}

fn fill_grid(
    sx: i32,
    sy: i32,
    grid: &[u8],
    width: usize,
    height: usize,
    seen: &mut [bool],
) -> FillGridResult {
    let mut area = 0;
    let mut horizonzal_sides = Vec::new();
    let mut vertical_sides = Vec::new();

    let mut queue = VecDeque::new();
    queue.push_back((sx, sy, grid[sy as usize * width + sx as usize]));

    while let Some((x, y, c)) = queue.pop_front() {
        if seen[y as usize * width + x as usize] {
            continue;
        }
        seen[y as usize * width + x as usize] = true;

        area += 1;

        for d in [(1, 0), (-1, 0), (0, 1), (0, -1)] {
            let nx = x + d.0;
            let ny = y + d.1;
            if nx >= 0
                && ny >= 0
                && nx < width as i32
                && ny < height as i32
                && grid[ny as usize * width + nx as usize] == c
            {
                queue.push_back((nx, ny, c));
            } else if d == (1, 0) || d == (-1, 0) {
                vertical_sides.push((y, x * 4 + d.0));
            } else {
                horizonzal_sides.push((x, y * 4 + d.1));
            }
        }
    }

    FillGridResult {
        area,
        horizonzal_sides,
        vertical_sides,
    }
}

fn remove_connected(s: (i32, i32), sides: &mut Vec<(i32, i32)>) {
    for d in [(1, 0), (-1, 0)] {
        let na = s.0 + d.0;
        let nb = s.1 + d.1;
        let k = sides.iter().position(|p| p.0 == na && p.1 == nb);
        if let Some(k) = k {
            sides.swap_remove(k);
            remove_connected((na, nb), sides);
        }
    }
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

    let mut seen = vec![false; grid.len()];
    let mut total1 = 0;
    let mut total2 = 0;
    for y in 0..height as i32 {
        for x in 0..width as i32 {
            if seen[y as usize * width + x as usize] {
                continue;
            }

            let fgr = fill_grid(x, y, &grid, width, height, &mut seen);
            total1 += fgr.area * (fgr.horizonzal_sides.len() + fgr.vertical_sides.len());

            let mut n_sides = 0;
            for sides in [fgr.horizonzal_sides, fgr.vertical_sides].iter_mut() {
                while !sides.is_empty() {
                    let s = sides.swap_remove(0);
                    remove_connected(s, sides);
                    n_sides += 1;
                }
            }

            total2 += fgr.area * n_sides;
        }
    }

    println!("{}", total1);
    println!("{}", total2);
}
