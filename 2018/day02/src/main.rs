use std::fs;

use rustc_hash::FxHashMap;

fn main() {
    let input = fs::read_to_string("input.txt").expect("Could not read file");
    let words = input
        .lines()
        .map(|l| l.bytes().collect::<Vec<_>>())
        .collect::<Vec<_>>();

    // part 1
    let mut twos = 0;
    let mut threes = 0;
    for w in words.iter() {
        let mut counts = [0; 26];
        for &c in w {
            counts[(c - b'a') as usize] += 1;
        }
        if counts.contains(&2) {
            twos += 1;
        }
        if counts.contains(&3) {
            threes += 1;
        }
    }
    println!("{}", twos * threes);

    // part 2 - The differing character must be in the first or second half of
    // the string and both strings must have the same length. This allows us to
    // create an index to significantly reduce the search space.
    let mut map: FxHashMap<&[u8], Vec<&[u8]>> = FxHashMap::default();
    for w in &words {
        let first_half = &w[0..w.len() / 2];
        let second_half = &w[w.len() / 2..];
        map.entry(first_half).or_default().push(w);
        map.entry(second_half).or_default().push(w);
    }

    'outer: for w1 in &words {
        let first_candidates = map.get(&w1[0..w1.len() / 2]).unwrap();
        let second_candidates = map.get(&w1[w1.len() / 2..]).unwrap();
        for w2 in first_candidates.iter().chain(second_candidates) {
            if w1.len() != w2.len() {
                continue;
            }

            let mut diffs = 0;
            let mut diff_index = 0;
            for (i, &c) in w1.iter().enumerate() {
                if c != w2[i] {
                    diffs += 1;
                    diff_index = i;
                    if diffs > 1 {
                        break;
                    }
                }
            }

            if diffs == 1 {
                println!(
                    "{}{}",
                    str::from_utf8(&w1[..diff_index]).unwrap(),
                    str::from_utf8(&w1[diff_index + 1..]).unwrap(),
                );
                break 'outer;
            }
        }
    }
}
