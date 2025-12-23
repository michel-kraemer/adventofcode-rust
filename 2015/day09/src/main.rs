use std::fs;

/// Computes the next permutation of a bitmask where `k` out of `n` bits are
/// set, in lexicographical order. For example, if `k` is 3 and the current
/// bitmask is 00010011, the next items would be 00010101, 00010110, 00011001,
/// etc.
///
/// See https://graphics.stanford.edu/~seander/bithacks.html#NextBitPermutation
fn next_permutation(mask: usize, n: u32) -> Option<usize> {
    if n - mask.count_ones() == mask.trailing_zeros() {
        // there is no next permutation
        None
    } else {
        let t = mask | (mask - 1);
        Some((t + 1) | (((!t & (!t).wrapping_neg()) - 1) >> (mask.trailing_zeros() + 1)))
    }
}

/// Apply a modified Held–Karp algorithm to find the shortest Hamiltonian path
/// in the given graph of distances. The modification is as follows:
///
/// * We do not always start at city 0 and thus do not initialize the DP table
/// * We iterate over all subset sizes `s` from `2..=n_cities` (and not just
///   `2..=n_cities - 1`)
/// * In each iteration over `s`, we iterate over all cities instead of always
///   skipping the first one
/// * We do not close the path at the end (i.e. we do not return to city 0)
/// * At the end, the shortest path is the minimum value in the last DP entry
///   (the entry where all cities have been visited)
///
/// For details, compare the implementation with the pseudo-code on the
/// Wikipedia page (https://en.wikipedia.org/wiki/Held%E2%80%93Karp_algorithm)
/// and the solution to day 13 where we use a non-modified version of the
/// Held–Karp algorithm.
///
/// The function accepts `min` parameter, which can be set to `true` to find the
/// shortest path and `false` to find the `longest` one.
fn find(distances: &[(&str, &str, u64)], n_cities: usize, min: bool) -> u64 {
    let mut dp = vec![vec![0; n_cities]; 1 << n_cities];

    for s in 2..=n_cities {
        let mut mask: usize = (1 << s) - 1;
        loop {
            let mut km = mask;
            while km > 0 {
                // select LSB and reset it
                let k = km.trailing_zeros() as usize;
                km &= km - 1;

                let mask_without_k = mask & !(1 << k);

                let mut v = if min { u64::MAX } else { 0 };
                let mut mm = mask_without_k;
                while mm > 0 {
                    let m = mm.trailing_zeros() as usize;
                    mm &= mm - 1;

                    let d = dp[mask_without_k][m] + distances[m * n_cities + k].2;
                    v = if min { v.min(d) } else { v.max(d) };
                }

                dp[mask][k] = v;
            }

            let Some(next) = next_permutation(mask, n_cities as u32) else {
                break;
            };
            mask = next;
        }
    }

    let ri = dp[dp.len() - 1].iter();
    if min {
        *ri.min().unwrap()
    } else {
        *ri.max().unwrap()
    }
}

fn main() {
    let input = fs::read_to_string("input.txt").expect("Could not read file");

    // parse input and create list of all distances between all cities
    let mut distances = Vec::new();
    for l in input.lines() {
        let mut parts = l.split_ascii_whitespace();
        let from = parts.next().unwrap();
        let to = parts.nth(1).unwrap();
        let dist = parts.nth(1).unwrap().parse::<u64>().unwrap();
        distances.push((from, to, dist));
        distances.push((to, from, dist));
        distances.push((to, to, 0));
        distances.push((from, from, 0));
    }

    // sort distances alphabetically by (from, to)
    distances.sort_unstable_by_key(|c| (c.0, c.1));
    distances.dedup();

    // determine number of cities
    let mut n_cities = 1;
    while n_cities * n_cities < distances.len() {
        n_cities += 1;
    }

    println!("{}", find(&distances, n_cities, true));
    println!("{}", find(&distances, n_cities, false));
}
