use std::collections::HashMap;
use std::fs;

fn blink(n: u64, i: usize, cache: &mut HashMap<(u64, usize), usize>) -> usize {
    if i == 0 {
        1
    } else if n == 0 {
        blink(1, i - 1, cache)
    } else {
        let ndigits = n.ilog10() + 1;
        if ndigits % 2 == 0 {
            // only caching stones with an even number of digits leads to fewer
            // hash map lookups and is apparently beneficial for performance
            if let Some(cached) = cache.get(&(n, i)) {
                *cached
            } else {
                let mask = 10u64.pow(ndigits / 2);
                let l = n / mask;
                let r = n % mask;
                let s = blink(l, i - 1, cache) + blink(r, i - 1, cache);
                cache.insert((n, i), s);
                s
            }
        } else {
            blink(n * 2024, i - 1, cache)
        }
    }
}

fn main() {
    for part1 in [true, false] {
        let input = fs::read_to_string("input.txt").expect("Could not read file");

        let mut total = 0;
        let mut cache = HashMap::with_capacity(5000);
        for s in input.split_whitespace() {
            total += blink(s.parse().unwrap(), if part1 { 25 } else { 75 }, &mut cache);
        }

        println!("{}", total);
    }
}
