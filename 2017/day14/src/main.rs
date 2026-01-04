use std::{collections::VecDeque, fs};

fn hash(input: &[usize], rowi: usize, row: &mut [bool; 128]) {
    let mut lengths = input.to_vec();
    lengths.push(b'-' as usize);

    let digits = if rowi > 0 {
        (rowi.ilog10() + 1) as usize
    } else {
        1
    };
    let ll = lengths.len() + digits;
    lengths.resize(ll, b'0' as usize);
    let mut n = rowi;
    for j in 0..digits {
        lengths[ll - 1 - j] = n % 10 + b'0' as usize;
        n /= 10;
    }
    lengths.extend(&[17, 31, 73, 47, 23]);

    let mut h = Vec::from_iter(0u8..=255u8);
    let mut i = 0;
    let mut skip = 0;

    for _ in 0..64 {
        for l in &lengths {
            for j in 0..l / 2 {
                let j1 = (i + j) % h.len();
                let j2 = (i + (l - j - 1)) % h.len();
                h.swap(j1, j2);
            }
            i = (i + l + skip) % h.len();
            skip += 1;
        }
    }

    let mut j = 0;
    for c in h.chunks(16) {
        let b = c.iter().copied().reduce(|a, b| a ^ b).unwrap();
        for k in (0..8).rev() {
            row[j] = b & (1 << k) > 0;
            j += 1;
        }
    }
}

fn main() {
    let input = fs::read_to_string("input.txt").expect("Could not read file");
    let input = input.trim();

    let mut grid = [[false; 128]; 128];

    let ib = input.bytes().map(|c| c as usize).collect::<Vec<_>>();
    for (i, row) in grid.iter_mut().enumerate() {
        hash(&ib, i, row);
    }

    // part 1
    println!("{}", grid.iter().flatten().copied().filter(|b| *b).count());

    // part 2
    let mut groups = 0;
    let mut y = 0;
    let mut x = 0;
    loop {
        // find next start point
        let mut s = None;
        'outer: while y < 128 {
            while x < 128 {
                if grid[y][x] {
                    s = Some((x, y));
                    break 'outer;
                }
                x += 1;
            }
            y += 1;
            x = 0;
        }

        if let Some(s) = s {
            // we were able to find a start point
            groups += 1;

            // flood-fill group
            let mut queue = VecDeque::new();
            queue.push_back(s);
            while !queue.is_empty() {
                let c = queue.pop_front().unwrap();
                grid[c.1][c.0] = false;
                if c.0 > 0 && grid[c.1][c.0 - 1] {
                    queue.push_back((c.0 - 1, c.1));
                }
                if c.1 > 0 && grid[c.1 - 1][c.0] {
                    queue.push_back((c.0, c.1 - 1));
                }
                if c.0 < grid[0].len() - 1 && grid[c.1][c.0 + 1] {
                    queue.push_back((c.0 + 1, c.1));
                }
                if c.1 < grid.len() - 1 && grid[c.1 + 1][c.0] {
                    queue.push_back((c.0, c.1 + 1));
                }
            }
        } else {
            break;
        }
    }

    println!("{groups}");
}
