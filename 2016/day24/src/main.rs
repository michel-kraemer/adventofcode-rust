use std::{collections::VecDeque, fs};

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

/// Apply the Held–Karp algorithm
/// (https://en.wikipedia.org/wiki/Held%E2%80%93Karp_algorithm) to find the
/// shortest Hamiltonian cycle in the given distance graph. The function returns
/// both the shortest cycle (for part 2) and the shortest path starting at the
/// element at index 0 and visiting all other elements (for part 1).
fn held_karp(distances: &[i64], n_distances: usize) -> (i64, i64) {
    let mut dp = vec![vec![0; n_distances]; 1 << n_distances];

    for k in 1..n_distances {
        dp[1 << k][k] = distances[k];
    }

    for s in 2..n_distances {
        let mut permutation: usize = (1 << s) - 1;
        loop {
            let mask = permutation << 1;

            let mut km = mask;
            while km > 0 {
                // select LSB and reset it
                let k = km.trailing_zeros() as usize;
                km &= km - 1;

                let mask_without_k = mask & !(1 << k);

                let mut v = i64::MAX;
                let mut mm = mask_without_k;
                while mm > 0 {
                    let m = mm.trailing_zeros() as usize;
                    mm &= mm - 1;

                    let d = dp[mask_without_k][m] + distances[m * n_distances + k];
                    v = v.min(d)
                }

                dp[mask][k] = v;
            }

            let Some(next) = next_permutation(permutation, n_distances as u32 - 1) else {
                break;
            };
            permutation = next;
        }
    }

    let mut result_path = i64::MAX;
    let mut result_cycle = i64::MAX;
    for k in 1..n_distances {
        let o = dp[(1 << n_distances) - 2][k];
        result_path = result_path.min(o);
        result_cycle = result_cycle.min(o + distances[k]);
    }
    (result_path, result_cycle)
}

fn main() {
    let input = fs::read_to_string("input.txt").expect("Could not read file");

    let lines = input.lines().collect::<Vec<_>>();
    let width = lines[0].len();
    let height = lines.len();
    let grid = lines.iter().flat_map(|l| l.bytes()).collect::<Vec<_>>();

    let mut digits = Vec::new();
    let mut zero_index = 0;
    for y in 0..height {
        for x in 0..width {
            let c = grid[y * width + x];
            if c.is_ascii_digit() {
                if c == b'0' {
                    zero_index = digits.len();
                }
                digits.push((x, y));
            }
        }
    }

    // Make sure '0' is the first element. This is necessary for our
    // implementation of the Held–Karp algorithm (see find() function)
    digits.swap(0, zero_index);

    // Perform multiple BFSs to get the distances between all pairs of digits.
    // Minor performance optimization: since we always store the distance in
    // both directions, we can skip the last digit.
    let mut distances = vec![0; digits.len() * digits.len()];
    let mut queue = VecDeque::new();
    let mut seen = vec![false; width * height];
    for (di, &d) in digits.iter().enumerate().take(digits.len() - 1) {
        seen.fill(false);

        queue.push_back((d.0, d.1, 0));
        seen[d.1 * width + d.0] = true;

        while let Some((x, y, steps)) = queue.pop_front() {
            if grid[y * width + x].is_ascii_digit() {
                let oi = digits.iter().position(|o| *o == (x, y)).unwrap();
                distances[di * digits.len() + oi] = steps;
                distances[oi * digits.len() + di] = steps;
            }
            for dir in [(-1, 0), (1, 0), (0, -1), (0, 1)] {
                let nx = x.checked_add_signed(dir.0).unwrap();
                let ny = y.checked_add_signed(dir.1).unwrap();
                if grid[ny * width + nx] != b'#' && !seen[ny * width + nx] {
                    seen[ny * width + nx] = true;
                    queue.push_back((nx, ny, steps + 1));
                }
            }
        }
    }

    // Perform Held–Karp algorithm to find the shortest Hamiltonian cycle. Also
    // return the shortest path starting at the element at index 0 (our digit
    // '0', see above) and visiting all other digits.
    let (total1, total2) = held_karp(&distances, digits.len());
    println!("{total1}");
    println!("{total2}");
}
