use std::fs;

use rustc_hash::FxHashMap;

fn main() {
    let input = fs::read_to_string("input.txt").expect("Could not read file");
    let lines = input
        .lines()
        .map(|l| l.strip_prefix('+').unwrap_or(l).parse::<i32>().unwrap())
        .collect::<Vec<_>>();

    // part 1
    let total1 = lines.iter().sum::<i32>();
    println!("{total1}");

    // part 2 - `total1` is the increment that will be added to every frequency
    // in each round. One way to find the first repeating frequency would be as
    // follows: iterate through all frequencies `a` in the first round and find
    // another frequency `b` from the first round such that
    // `(a - b) % total1 == 0`, which means that it takes `n = (a - b) / total1`
    // rounds for `b` to become high enough to equal `a`. This might apply to
    // multiple `b`s, so find the one with the lowest `n` and the lowest index,
    // i.e. the first one that will become high enough.
    //
    // We speed this approach up by first collecting all frequencies from the
    // first round as well as their remainders when divided by `total1`. We then
    // iterate through the frequencies again and find all other frequencies
    // where the remainder matches. The one with the lowest `n` and the lowest
    // index (in this order) will be value we're looking for.
    if total1 == 0 {
        println!("0");
    } else {
        let mut frequencies = Vec::new();
        let mut remainders: FxHashMap<i32, Vec<usize>> = FxHashMap::default();
        let mut sum = 0;
        for (i, &l) in lines.iter().enumerate() {
            sum += l;
            frequencies.push(sum);
            remainders
                .entry(sum.rem_euclid(total1))
                .or_default()
                .push(i);
        }

        let mut min = (i32::MAX, usize::MAX);
        let mut minf = 0;
        for (i, &f) in frequencies.iter().enumerate() {
            let r = f % total1;
            if let Some(js) = remainders.get(&r) {
                for &j in js {
                    if j == i {
                        continue;
                    }
                    let n = (f - frequencies[j]) / total1;
                    // n must be higher than 0 (which means the other frequency
                    // doesn't come from the first round), or if it's 0 (first
                    // round), then its index must at least be higher than the
                    // index of the current frequency. Only then, we compare `n`
                    // and the index with `min`.
                    if (n > 0 || (n == 0 && j > i)) && (n, j) < min {
                        min = (n, j);
                        minf = f;
                    }
                }
            }
        }
        println!("{minf}");
    }
}
