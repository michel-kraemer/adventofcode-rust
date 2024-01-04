use std::{collections::HashMap, fs};

fn main() {
    let input = fs::read_to_string("input.txt").expect("Could not read file");
    let mut banks = input
        .split_whitespace()
        .map(|l| l.parse::<i32>().unwrap())
        .collect::<Vec<_>>();

    let mut steps = 0;
    let cycle_len;
    let mut seen = HashMap::new();

    loop {
        if seen.contains_key(&banks) {
            cycle_len = steps - seen[&banks];
            break;
        }
        seen.insert(banks.clone(), steps);

        let mut max = *banks.iter().max().unwrap();
        let i = banks.iter().position(|&v| v == max).unwrap();

        banks[i] = 0;

        let mut j = 0;
        while max > 0 {
            let k = (i + j + 1) % banks.len();
            banks[k] += 1;
            max -= 1;
            j += 1;
        }

        steps += 1;
    }

    println!("{}", steps);
    println!("{}", cycle_len);
}
