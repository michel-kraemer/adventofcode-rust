use std::collections::VecDeque;
use std::fs;

struct Region {
    area: usize,
    horizonzal_sides: Vec<(i32, i32)>,
    vertical_sides: Vec<(i32, i32)>,
}

fn fill_region(
    sx: i32,
    sy: i32,
    grid: &[u8],
    width: usize,
    height: usize,
    seen: &mut [bool],
) -> Region {
    let mut area = 0;
    let mut horizonzal_sides = Vec::new();
    let mut vertical_sides = Vec::new();

    let mut queue = VecDeque::new();
    queue.push_back((sx, sy));
    seen[sy as usize * width + sx as usize] = true;

    while let Some((x, y)) = queue.pop_front() {
        let c = grid[y as usize * width + x as usize];
        area += 1;

        for d in [(1, 0), (-1, 0), (0, 1), (0, -1)] {
            let nx = x + d.0;
            let ny = y + d.1;
            let ni = ny as usize * width + nx as usize;
            if nx >= 0 && ny >= 0 && nx < width as i32 && ny < height as i32 && grid[ni] == c {
                if !seen[ni] {
                    seen[ni] = true;
                    queue.push_back((nx, ny));
                }
            } else if d.1 == 0 {
                vertical_sides.push((y, x * 4 + d.0));
            } else {
                horizonzal_sides.push((x, y * 4 + d.1));
            }
        }
    }

    Region {
        area,
        horizonzal_sides,
        vertical_sides,
    }
}

fn remove_connected(s: (i32, i32), sides: &mut Vec<(i32, i32)>) {
    // since there's always only a very small number of side tiles, it's
    // faster to use a Vec instead of a HashSet or a BinaryHeap
    let mut a = s.0 + 1;
    while let Some(k) = sides.iter().position(|p| p.0 == a && p.1 == s.1) {
        sides.swap_remove(k);
        a += 1;
    }
    let mut a = s.0 - 1;
    while let Some(k) = sides.iter().position(|p| p.0 == a && p.1 == s.1) {
        sides.swap_remove(k);
        a -= 1;
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

            // Fill region. This will give us its area and all its horizontal
            // and vertical side tiles.
            let region = fill_region(x, y, &grid, width, height, &mut seen);
            total1 += region.area * (region.horizonzal_sides.len() + region.vertical_sides.len());

            // find connected side tiles and count how many sides there are
            let mut n_sides = 0;
            for sides in [region.horizonzal_sides, region.vertical_sides].iter_mut() {
                while !sides.is_empty() {
                    let s = sides.swap_remove(0);
                    remove_connected(s, sides);
                    n_sides += 1;
                }
            }

            total2 += region.area * n_sides;
        }
    }

    println!("{}", total1);
    println!("{}", total2);
}
