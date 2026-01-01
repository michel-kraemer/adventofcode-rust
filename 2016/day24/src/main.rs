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
fn held_karp(distances: &[Vec<i64>]) -> (i64, i64) {
    let mut dp = vec![vec![0; distances.len()]; 1 << distances.len()];

    for (k, &dist) in distances[0].iter().enumerate().skip(1) {
        dp[1 << k][k] = dist;
    }

    for s in 2..distances.len() {
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

                    let d = dp[mask_without_k][m] + distances[m][k];
                    v = v.min(d)
                }

                dp[mask][k] = v;
            }

            let Some(next) = next_permutation(permutation, distances.len() as u32 - 1) else {
                break;
            };
            permutation = next;
        }
    }

    let mut result_path = i64::MAX;
    let mut result_cycle = i64::MAX;
    for (k, &dist) in distances[0].iter().enumerate().skip(1) {
        let o = dp[(1 << distances.len()) - 2][k];
        result_path = result_path.min(o);
        result_cycle = result_cycle.min(o + dist);
    }
    (result_path, result_cycle)
}

fn main() {
    let input = fs::read_to_string("input.txt").expect("Could not read file");

    let grid = input
        .lines()
        .map(|l| l.bytes().collect::<Vec<_>>())
        .collect::<Vec<_>>();
    let width = grid[0].len();
    let height = grid.len();

    let mut digits = Vec::new();
    let mut zero_index = 0;
    for (y, row) in grid.iter().enumerate() {
        for (x, cell) in row.iter().enumerate() {
            if cell.is_ascii_digit() {
                if *cell == b'0' {
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
    let mut distances = vec![vec![0; digits.len()]; digits.len()];
    let mut queue = VecDeque::new();
    let mut seen = vec![vec![false; width]; height];
    for (di, &d) in digits.iter().enumerate().take(digits.len() - 1) {
        seen.iter_mut().for_each(|row| row.fill(false));

        queue.push_back((d.0, d.1, 0));
        seen[d.1][d.0] = true;

        while let Some((x, y, steps)) = queue.pop_front() {
            if grid[y][x].is_ascii_digit() {
                let oi = digits.iter().position(|o| *o == (x, y)).unwrap();
                distances[di][oi] = steps;
                distances[oi][di] = steps;
            }
            for dir in [(-1, 0), (1, 0), (0, -1), (0, 1)] {
                let nx = x.checked_add_signed(dir.0).unwrap();
                let ny = y.checked_add_signed(dir.1).unwrap();
                if grid[ny][nx] != b'#' && !seen[ny][nx] {
                    seen[ny][nx] = true;
                    queue.push_back((nx, ny, steps + 1));
                }
            }
        }
    }

    // Perform Held–Karp algorithm to find the shortest Hamiltonian cycle. Also
    // return the shortest path starting at the element at index 0 (our digit
    // '0', see above) and visiting all other digits.
    let (total1, total2) = held_karp(&distances);
    println!("{total1}");
    println!("{total2}");
}
