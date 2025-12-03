use std::fs;

fn main() {
    let input = fs::read_to_string("input.txt").expect("Could not read file");

    // parse ranges and find maximum length of numbers
    let mut max_len = 0;
    let mut ranges = Vec::new();
    for range in input.trim().split(",") {
        let (lo, hi) = range.split_once("-").unwrap();
        max_len = max_len.max(lo.len());
        max_len = max_len.max(hi.len());
        let lo = lo.parse::<u64>().unwrap();
        let hi = hi.parse::<u64>().unwrap();
        ranges.push(lo..=hi);
    }

    // ranges are non-overlapping, so we can sort them
    ranges.sort_by_key(|r| *r.start());

    // Generate all possible numbers with repeated sequences of digits. Keep
    // track of those that only consist of two sequences (for part 1).
    let mut all = Vec::new();
    for l in 2..=max_len {
        let mut mul: u64 = 10;
        for sl in 1..=l / 2 {
            if l.is_multiple_of(sl) {
                for i in mul / 10..mul {
                    let mut n = 0;
                    for _ in 0..l / sl {
                        n *= mul;
                        n += i;
                    }
                    all.push((n, !(l.is_multiple_of(2) && sl == l / 2)));
                }
            }
            mul *= 10;
        }
    }

    // sort() is faster than sort_unstable() in this case because the majority
    // of the numbers are already sorted (see documentation of sort())
    all.sort();
    all.dedup_by_key(|a| a.0);

    // check which of the numbers in `all` lie in any of the ranges
    let mut ri = ranges.iter();
    let mut current_range = ri.next().unwrap();
    let mut total1 = 0;
    let mut total2 = 0;
    for (a, odd) in &all {
        if current_range.end() < a {
            let Some(cr) = ri.find(|r| r.end() >= a) else {
                break;
            };
            current_range = cr;
        }
        if current_range.contains(a) {
            if !odd {
                total1 += a;
            }
            total2 += a;
        }
    }

    println!("{total1}");
    println!("{total2}");
}
