use std::{collections::VecDeque, fs};

fn diri(b: (i32, i32, i32, i32)) -> usize {
    if b.2 == 1 {
        0
    } else if b.2 == -1 {
        1
    } else if b.3 == 1 {
        2
    } else {
        3
    }
}

fn energize(start: (i32, i32, i32, i32), grid: &[u8], w: usize, h: usize) -> usize {
    let mut queue = VecDeque::new();
    queue.push_back(start);

    let mut seen = vec![[false; 4]; grid.len()];
    let mut visited = vec![false; grid.len()];

    'outer: while let Some(mut b) = queue.pop_front() {
        visited[b.1 as usize * w + b.0 as usize] = true;

        // performance optimization shortcut: continue straight as far as possible
        while grid[b.1 as usize * w + b.0 as usize] == b'.' {
            b.0 += b.2;
            b.1 += b.3;
            if b.0 < 0 || b.1 < 0 || b.0 == w as i32 || b.1 == h as i32 {
                continue 'outer;
            }
            visited[b.1 as usize * w + b.0 as usize] = true;
        }

        let (nb1, nb2) = match grid[b.1 as usize * w + b.0 as usize] {
            b'/' => (Some((b.0, b.1, -b.3, -b.2)), None),
            b'\\' => (Some((b.0, b.1, b.3, b.2)), None),
            b'|' => {
                if b.2 != 0 {
                    (Some((b.0, b.1, 0, -1)), Some((b.0, b.1, 0, 1)))
                } else {
                    (Some(b), None)
                }
            }
            b'-' => {
                if b.3 != 0 {
                    (Some((b.0, b.1, -1, 0)), Some((b.0, b.1, 1, 0)))
                } else {
                    (Some(b), None)
                }
            }
            _ => unreachable!(),
        };

        for mut nb in [nb1, nb2].into_iter().flatten() {
            nb.0 += nb.2;
            nb.1 += nb.3;
            let di = diri(nb);
            if nb.0 >= 0
                && nb.1 >= 0
                && nb.0 < w as i32
                && nb.1 < h as i32
                && !seen[nb.1 as usize * w + nb.0 as usize][di]
            {
                seen[nb.1 as usize * w + nb.0 as usize][di] = true;
                queue.push_back(nb);
            }
        }
    }

    visited.into_iter().filter(|&v| v).count()
}

fn main() {
    let input = fs::read_to_string("input.txt").expect("Could not read file");
    let lines: Vec<&str> = input.lines().collect();
    let grid: Vec<u8> = lines.iter().flat_map(|&l| l.as_bytes().to_vec()).collect();
    let w = lines[0].len();
    let h = lines.len();

    let total = energize((0, 0, 1, 0), &grid, w, h);
    println!("{}", total);

    let mut max = 0;
    for x in 0..w as i32 {
        max = max.max(energize((x, 0, 0, 1), &grid, w, h));
        max = max.max(energize((x, h as i32 - 1, 0, -1), &grid, w, h));
    }
    for y in 0..h as i32 {
        max = max.max(energize((0, y, 1, 0), &grid, w, h));
        max = max.max(energize((w as i32 - 1, y, -1, 0), &grid, w, h));
    }

    println!("{}", max);
}
