use std::collections::HashMap;
use std::fs;

fn apply(n: u64, i: usize, seen: &mut HashMap<(u64, usize), usize>) -> usize {
    if i == 0 {
        return 1;
    } else if let Some(cached) = seen.get(&(n, i)) {
        return *cached;
    }

    let r = if n == 0 {
        apply(1, i - 1, seen)
    } else {
        let ndigits = n.ilog10() + 1;
        if ndigits % 2 == 0 {
            let mask = 10u64.pow(ndigits / 2);
            let l = n / mask;
            let r = n % mask;
            apply(l, i - 1, seen) + apply(r, i - 1, seen)
        } else {
            apply(n * 2024, i - 1, seen)
        }
    };

    seen.insert((n, i), r);
    r
}

fn main() {
    for part1 in [true, false] {
        let input = fs::read_to_string("input.txt").expect("Could not read file");

        let mut total = 0;
        let mut seen = HashMap::new();
        for s in input.split_whitespace() {
            total += apply(s.parse().unwrap(), if part1 { 25 } else { 75 }, &mut seen);
        }

        println!("{}", total);
    }
}
