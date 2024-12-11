use std::collections::HashMap;
use std::fs;

fn blink(
    n: u64,
    i: usize,
    cache: &mut HashMap<(u64, usize), usize>,
    fast_cache: &mut [usize; 75 * 10000],
) -> usize {
    if i == 0 {
        1
    } else if n == 0 {
        blink(1, i - 1, cache, fast_cache)
    } else {
        let ndigits = n.ilog10() + 1;
        if ndigits % 2 == 0 {
            // only caching stones with an even number of digits leads to fewer
            // hash map lookups and is apparently beneficial for performance
            if ndigits < 5 && i <= 75 {
                if fast_cache[n as usize * 75 + i - 1] != usize::MAX {
                    return fast_cache[n as usize * 75 + i - 1];
                }
            } else if let Some(cached) = cache.get(&(n, i)) {
                return *cached;
            }

            let mask = 10u64.pow(ndigits / 2);
            let l = n / mask;
            let r = n % mask;
            let s = blink(l, i - 1, cache, fast_cache) + blink(r, i - 1, cache, fast_cache);

            if ndigits < 5 && i <= 75 {
                fast_cache[n as usize * 75 + i - 1] = s;
            } else {
                cache.insert((n, i), s);
            }

            s
        } else {
            blink(n * 2024, i - 1, cache, fast_cache)
        }
    }
}

fn main() {
    let input = fs::read_to_string("input.txt").expect("Could not read file");

    let mut cache = HashMap::with_capacity(5000);

    // A cache for numbers smaller than 10000 (which is the majority). Reduces
    // the number of hash map lookups and saves time.
    let mut fast_cache = [usize::MAX; 75 * 10000];

    for part1 in [true, false] {
        let mut total = 0;
        for s in input.split_whitespace() {
            total += blink(
                s.parse().unwrap(),
                if part1 { 25 } else { 75 },
                &mut cache,
                &mut fast_cache,
            );
        }

        println!("{}", total);
    }
}
