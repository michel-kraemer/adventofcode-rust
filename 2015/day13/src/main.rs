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

/// Apply the Held–Karp algorithm to find the longest Hamiltonian path in the
/// given graph of gains and losses.
///
/// This is similar to day 9, but here, we're using the actual Held–Karp
/// algorithm and not a modified one.
///
/// The parameter `n_people` specifies for how many people the longest path
/// should be calculated, whereas `total_people` specifies how many people exist
/// in total in the table of gains and losses. Differentiating between those two
/// allows us to calculate the path without ME (for part 1).
///
/// For more information, see the description of the algorithm at Wikipedia:
/// https://en.wikipedia.org/wiki/Held%E2%80%93Karp_algorithm
fn find(gains: &[i64], n_people: usize, total_people: usize) -> i64 {
    let mut dp = vec![vec![0; n_people]; 1 << n_people];

    for k in 1..n_people {
        dp[1 << k][k] = gains[k];
    }

    for s in 2..n_people {
        let mut permutation: usize = (1 << s) - 1;
        loop {
            let mask = permutation << 1;

            let mut km = mask;
            while km > 0 {
                // select LSB and reset it
                let k = km.trailing_zeros() as usize;
                km &= km - 1;

                let mask_without_k = mask & !(1 << k);

                let mut v = i64::MIN;
                let mut mm = mask_without_k;
                while mm > 0 {
                    let m = mm.trailing_zeros() as usize;
                    mm &= mm - 1;

                    let d = dp[mask_without_k][m] + gains[m * total_people + k];
                    v = v.max(d)
                }

                dp[mask][k] = v;
            }

            let Some(next) = next_permutation(permutation, n_people as u32 - 1) else {
                break;
            };
            permutation = next;
        }
    }

    let mut result = i64::MIN;
    for k in 1..n_people {
        result = result.max(dp[(1 << n_people) - 2][k] + gains[k]);
    }
    result
}

fn main() {
    // ME should have a name that comes after all others (alphabetically), so we
    // can skip ME in part 1 by passing `n_people-1` to `find`
    const ME: &str = "\x7f";

    let input = fs::read_to_string("input.txt").expect("Could not read file");

    // parse input and create list of all gains and losses
    let mut gains = Vec::new();
    for l in input.lines() {
        let mut parts = l.split_ascii_whitespace();
        let name = parts.next().unwrap();
        let gainlose = parts.nth(1).unwrap();
        let mut points = parts.next().unwrap().parse::<i64>().unwrap();
        if gainlose == "lose" {
            points = -points;
        }
        let other = parts.nth(6).unwrap();
        let other = &other[..other.len() - 1];

        gains.push((name, other, points));
        gains.push((other, name, points));
        gains.push((name, name, 0));
        gains.push((other, other, 0));

        // insert ME
        gains.push((name, ME, 0));
        gains.push((ME, name, 0));
        gains.push((other, ME, 0));
        gains.push((ME, other, 0));
    }
    gains.push((ME, ME, 0));

    // sort gains alphabetically by (name, other)
    gains.sort_unstable_by_key(|c| (c.0, c.1));

    // deduplicate and sum up the gains and losses of each pair of people
    let mut merged_gains = Vec::new();
    let mut i = 0;
    while i < gains.len() {
        let mut current_sum = 0;
        let mut j = i;
        while j < gains.len() && (gains[j].0, gains[j].1) == (gains[i].0, gains[i].1) {
            current_sum += gains[j].2;
            j += 1;
        }
        merged_gains.push(current_sum);
        i = j;
    }

    // determine number of people
    let mut n_people = 1;
    while n_people * n_people < merged_gains.len() {
        n_people += 1;
    }

    // part 1 - calculate longest hamiltonian path but skip ME
    println!("{}", find(&merged_gains, n_people - 1, n_people));

    // part 2 - include ME
    println!("{}", find(&merged_gains, n_people, n_people));
}
