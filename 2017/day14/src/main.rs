use std::{collections::VecDeque, fs};

fn hash(s: &str) -> String {
    let mut lenghts = s.chars().map(|c| c as usize).collect::<Vec<_>>();
    lenghts.extend(&[17, 31, 73, 47, 23]);

    let mut h = Vec::from_iter(0u8..=255u8);
    let mut i = 0;
    let mut skip = 0;

    for _ in 0..64 {
        for l in &lenghts {
            for j in 0..l / 2 {
                let j1 = (i + j) % h.len();
                let j2 = (i + (l - j - 1)) % h.len();
                h.swap(j1, j2);
            }
            i = (i + l + skip) % h.len();
            skip += 1;
        }
    }

    h.chunks(16)
        .map(|c| c.iter().copied().reduce(|a, b| a ^ b).unwrap())
        .map(|b| format!("{:02x}", b))
        .collect::<String>()
}

fn main() {
    let input = fs::read_to_string("input.txt").expect("Could not read file");
    let input = input.trim();

    let mut grid = Vec::new();

    for i in 0..128 {
        let s = format!("{}-{}", input, i);
        let h = hash(&s);
        let mut r = Vec::new();
        for c in h.chars() {
            let v = c.to_digit(16).unwrap();
            r.push(v & 8 == 8);
            r.push(v & 4 == 4);
            r.push(v & 2 == 2);
            r.push(v & 1 == 1);
        }
        grid.push(r);
    }

    // part 1
    let c = grid.iter().flatten().filter(|b| **b).count();
    println!("{}", c);

    // part 2
    let mut groups = 0;
    loop {
        // find any start point
        let mut s = None;
        for y in 0..grid.len() {
            for x in 0..grid[0].len() {
                if grid[y][x] {
                    s = Some((x, y));
                    break;
                }
            }
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

    println!("{}", groups);
}
