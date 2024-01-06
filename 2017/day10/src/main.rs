use std::fs;

fn main() {
    for part1 in [true, false] {
        let input = fs::read_to_string("input.txt").expect("Could not read file");
        let mut lenghts = if part1 {
            input
                .trim()
                .split(',')
                .map(|n| n.trim().parse::<usize>().unwrap())
                .collect::<Vec<_>>()
        } else {
            input.trim().chars().map(|c| c as usize).collect::<Vec<_>>()
        };

        if !part1 {
            lenghts.extend(&[17, 31, 73, 47, 23]);
        }

        let mut h = Vec::from_iter(0u8..=255u8);
        let mut i = 0;
        let mut skip = 0;

        for _ in 0..(if part1 { 1 } else { 64 }) {
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

        if part1 {
            println!("{}", h[0] as i32 * h[1] as i32);
        } else {
            let dense = h
                .chunks(16)
                .map(|c| c.iter().copied().reduce(|a, b| a ^ b).unwrap())
                .collect::<Vec<_>>();
            let hex = dense
                .iter()
                .map(|b| format!("{:02x}", b))
                .collect::<String>();
            println!("{}", hex);
        }
    }
}
