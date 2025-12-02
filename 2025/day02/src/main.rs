use std::{collections::HashSet, fs};

fn main() {
    let input = fs::read_to_string("input.txt").expect("Could not read file");

    // parse ranges and find maximum length of numbers
    let mut max_len = 0;
    let mut ranges = Vec::new();
    for range in input.trim().split(",") {
        let (lo, hi) = range.split_once("-").unwrap();
        max_len = max_len.max(lo.len());
        max_len = max_len.max(hi.len());
        let lo = lo.parse::<i64>().unwrap();
        let hi = hi.parse::<i64>().unwrap();
        ranges.push(lo..=hi);
    }

    // Generate all possible numbers with repeated sequences of digits. Keep
    // track of those that only consist of two sequences (for part 1).
    let mut all_part1 = HashSet::new();
    let mut all = HashSet::new();
    let mut mul: i64 = 10;
    let mut ilen = 1;
    let mut chunks = max_len;
    for i in 1.. {
        if i == mul {
            mul *= 10;
            ilen += 1;
            chunks = max_len / ilen;
            if chunks == 1 {
                break;
            }
        }
        let mut m = i;
        for c in 0..chunks - 1 {
            m *= mul;
            m += i;
            if c == 0 {
                // number consists of only two sequences
                all_part1.insert(m);
            }
            all.insert(m);
        }
    }

    // check which of the numbers in `all` lie in any of the ranges
    let mut total1 = 0;
    let mut total2 = 0;
    for a in &all {
        if ranges.iter().any(|range| range.contains(a)) {
            if all_part1.contains(a) {
                total1 += a;
            }
            total2 += a;
        }
    }
    println!("{total1}");
    println!("{total2}");
}
