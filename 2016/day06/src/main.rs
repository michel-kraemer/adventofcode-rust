use std::fs;

fn main() {
    let input = fs::read_to_string("input.txt").expect("Could not read file");

    let mut counts = Vec::new();

    for l in input.lines() {
        if counts.is_empty() {
            counts = vec![vec![0; 26]; l.len()];
        }
        for (i, b) in l.bytes().enumerate() {
            counts[i][(b - b'a') as usize] += 1;
        }
    }

    println!(
        "{}",
        counts
            .iter()
            .map(|c| (c.iter().enumerate().max_by_key(|i| i.1).unwrap().0 as u8 + b'a') as char)
            .collect::<String>()
    );

    println!(
        "{}",
        counts
            .iter()
            .map(|c| (c.iter().enumerate().min_by_key(|i| i.1).unwrap().0 as u8 + b'a') as char)
            .collect::<String>()
    );
}
