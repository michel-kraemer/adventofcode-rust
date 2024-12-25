use std::fs;

fn main() {
    let input = fs::read_to_string("input.txt").expect("Could not read file");

    let grids = input.split("\n\n").collect::<Vec<_>>();

    let mut locks = Vec::new();
    let mut keys = Vec::new();

    // decide which grids are locks and which are keys and calculate pin heights
    let mut total_height = 0;
    for g in grids.iter() {
        let g = g
            .lines()
            .map(|l| l.chars().collect::<Vec<_>>())
            .collect::<Vec<_>>();
        let is_key = g[0][0] != '#';
        total_height = g.len();

        let mut heights = vec![0usize; g[0].len()];
        for row in g {
            for (x, c) in row.iter().enumerate() {
                if *c == '#' {
                    heights[x] += 1;
                }
            }
        }

        if is_key {
            keys.push(heights);
        } else {
            locks.push(heights);
        }
    }

    // check which key fits into which lock
    let mut total = 0;
    for k in &keys {
        for l in &locks {
            if k.iter().zip(l).all(|(a, b)| a + b <= total_height) {
                total += 1;
            }
        }
    }

    println!("{}", total);
}
