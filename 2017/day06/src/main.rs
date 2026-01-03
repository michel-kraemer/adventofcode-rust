use std::{collections::hash_map::Entry, fs};

use rustc_hash::{FxBuildHasher, FxHashMap};

fn main() {
    let input = fs::read_to_string("input.txt").expect("Could not read file");

    // the problem statement says there are exactly 16 banks, so we can use an
    // array instead of a Vec
    let mut banks: [u32; 16] = [0; 16];

    input
        .split_ascii_whitespace()
        .enumerate()
        .for_each(|(i, v)| banks[i] = v.parse().unwrap());

    let mut steps = 0;
    let mut seen = FxHashMap::with_capacity_and_hasher(1 << 14, FxBuildHasher);

    let cycle_len = loop {
        let entry = seen.entry(banks);
        if let Entry::Occupied(e) = entry {
            break steps - e.get();
        }
        entry.insert_entry(steps);

        let mut maxi = 0;
        let mut max = 0;
        for (i, &b) in banks.iter().enumerate() {
            if b > max {
                max = b;
                maxi = i;
            }
        }

        banks[maxi] = 0;
        let mut j = (maxi + 1) % banks.len();
        while max > 0 {
            banks[j] += 1;
            max -= 1;
            j = (j + 1) % banks.len();
        }

        steps += 1;
    };

    println!("{steps}");
    println!("{cycle_len}");
}
