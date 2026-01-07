//! We represent patterns as bitmasks, starting with the least-significant bit
//! in the upper left corner and continuing left to right and top to bottom. The
//! starting pattern
//!
//! ```plain
//! .#.
//! ..#
//! ###
//! ```
//!
//! becomes
//!
//! ```plain
//! .#./..#/###  =>  111 100 010
//! ```
//!
//! This allows us quickly map 2x2 and 3x3 patterns to enhanced 3x3 and 4x4
//! patterns, respectively, by looking up entries in a table.
//!
//! It also allows us to select subpatterns using bit operations. For example,
//! the following 4x4 pattern can be split into four 2x2 patterns:
//!
//! ```plain
//! Pattern:    Bit positions:     Splits:
//!                                 0  1 |  2  3
//! #..#         0  1  2  3         4  5 |  6  7
//! ....         4  5  6  7        ------|------
//! #..#         8  9 10 11         8  9 | 10 11
//! .##.        12 13 14 15        12 13 | 14 15
//!
//! Subpatterns:
//! a = (pattern & 0b11) | ((pattern >> 2) & 0b1100)          // bits   0 1 |  4  5
//! b = ((pattern >> 2) & 0b11) | ((pattern >> 4) & 0b1100)   // bits   2 3 |  6  7
//! c = ((pattern >> 8) & 0b11) | ((pattern >> 10) & 0b1100)  // bits   8 9 | 12 13
//! d = ((pattern >> 10) & 0b11) | ((pattern >> 12) & 0b1100) // bits 10 11 | 14 15
//! ```
//!
//! To improve parsing performance, I've generated look-up tables that map all
//! possible patterns to all their rotated and flipped counter-parts. These
//! tables are in the `lut` module. The code to generate them can also be found
//! in this module.
use std::fs;

use rustc_hash::{FxBuildHasher, FxHashMap};

use crate::lut::{LUT_2, LUT_3, UM};

mod lut;

/// Count the number of bits after enhancing a `pattern` of `size` 3x3, 4x4, or
/// 6x6 `n` times
///
/// The function uses a cache to quickly return previously computed results when
/// called with the same pattern. According to my analysis, cache hits seems to
/// happen for 3x3 patterns only.
fn count(
    pattern: usize,
    size: usize,
    n: usize,
    rules_2x2: &[usize; 16],
    rules_3x3: &[usize; 512],
    cache: &mut FxHashMap<(u32, u32), u64>,
) -> u64 {
    if n == 0 {
        return pattern.count_ones() as u64;
    }

    if size == 6 {
        // split the 6x6 pattern into nine 2x2 patterns
        let a = (pattern & 0b11) | ((pattern >> 4) & 0b1100);
        let b = ((pattern >> 2) & 0b11) | ((pattern >> 6) & 0b1100);
        let c = ((pattern >> 4) & 0b11) | ((pattern >> 8) & 0b1100);
        let d = ((pattern >> 12) & 0b11) | ((pattern >> 16) & 0b1100);
        let e = ((pattern >> 14) & 0b11) | ((pattern >> 18) & 0b1100);
        let f = ((pattern >> 16) & 0b11) | ((pattern >> 20) & 0b1100);
        let g = ((pattern >> 24) & 0b11) | ((pattern >> 28) & 0b1100);
        let h = ((pattern >> 26) & 0b11) | ((pattern >> 30) & 0b1100);
        let i = ((pattern >> 28) & 0b11) | ((pattern >> 32) & 0b1100);

        // map the nine 2x2 patterns to nine 3x3 patterns
        let new_a = rules_2x2[a];
        let new_b = rules_2x2[b];
        let new_c = rules_2x2[c];
        let new_d = rules_2x2[d];
        let new_e = rules_2x2[e];
        let new_f = rules_2x2[f];
        let new_g = rules_2x2[g];
        let new_h = rules_2x2[h];
        let new_i = rules_2x2[i];

        // recurse
        count(new_a, 3, n - 1, rules_2x2, rules_3x3, cache)
            + count(new_b, 3, n - 1, rules_2x2, rules_3x3, cache)
            + count(new_c, 3, n - 1, rules_2x2, rules_3x3, cache)
            + count(new_d, 3, n - 1, rules_2x2, rules_3x3, cache)
            + count(new_e, 3, n - 1, rules_2x2, rules_3x3, cache)
            + count(new_f, 3, n - 1, rules_2x2, rules_3x3, cache)
            + count(new_g, 3, n - 1, rules_2x2, rules_3x3, cache)
            + count(new_h, 3, n - 1, rules_2x2, rules_3x3, cache)
            + count(new_i, 3, n - 1, rules_2x2, rules_3x3, cache)
    } else if size == 4 {
        // split the 4x4 pattern into four 2x2 patterns
        let a = (pattern & 0b11) | ((pattern >> 2) & 0b1100);
        let b = ((pattern >> 2) & 0b11) | ((pattern >> 4) & 0b1100);
        let c = ((pattern >> 8) & 0b11) | ((pattern >> 10) & 0b1100);
        let d = ((pattern >> 10) & 0b11) | ((pattern >> 12) & 0b1100);

        // map the four 2x2 patterns to four 3x3 patterns
        let new_a = rules_2x2[a];
        let new_b = rules_2x2[b];
        let new_c = rules_2x2[c];
        let new_d = rules_2x2[d];

        // build a new 6x6 pattern out of the four 3x3 patterns
        let new_pattern = (new_a & 0b111)
            | ((new_b & 0b111) << 3)
            | ((new_a & (0b111 << 3)) << 3)
            | ((new_b & (0b111 << 3)) << 6)
            | ((new_a & (0b111 << 6)) << 6)
            | ((new_b & (0b111 << 6)) << 9)
            | ((new_c & 0b111) << 18)
            | ((new_d & 0b111) << 21)
            | ((new_c & (0b111 << 3)) << 21)
            | ((new_d & (0b111 << 3)) << 24)
            | ((new_c & (0b111 << 6)) << 24)
            | ((new_d & (0b111 << 6)) << 27);

        // recurse
        count(new_pattern, 6, n - 1, rules_2x2, rules_3x3, cache)
    } else {
        // 3x3 pattern
        if let Some(c) = cache.get(&(pattern as u32, n as u32)) {
            return *c;
        }

        let new_pattern = rules_3x3[pattern];
        let r = count(new_pattern, 4, n - 1, rules_2x2, rules_3x3, cache);

        cache.insert((pattern as u32, n as u32), r);

        r
    }
}

fn main() {
    let input = fs::read_to_string("input.txt").expect("Could not read file");
    let mut rules_2x2 = [0; 16];
    let mut rules_3x3 = [0; 512];

    for l in input.lines() {
        let p = l.split_once(" => ").unwrap();
        if p.0.len() == 5 {
            // parse 2x2 rule
            let b = p.0.as_bytes();
            let pattern = (b[0] == b'#') as usize
                | ((b[1] == b'#') as usize) << 1
                | ((b[3] == b'#') as usize) << 2
                | ((b[4] == b'#') as usize) << 3;

            // parse 3x3 replacement pattern
            let r = p.1.as_bytes();
            let replacement = (r[0] == b'#') as usize
                | ((r[1] == b'#') as usize) << 1
                | ((r[2] == b'#') as usize) << 2
                | ((r[4] == b'#') as usize) << 3
                | ((r[5] == b'#') as usize) << 4
                | ((r[6] == b'#') as usize) << 5
                | ((r[8] == b'#') as usize) << 6
                | ((r[9] == b'#') as usize) << 7
                | ((r[10] == b'#') as usize) << 8;

            // map all transformations of the rule to the replacement pattern
            for p in LUT_2[pattern] {
                if p == UM {
                    break;
                }
                rules_2x2[p as usize] = replacement;
            }
        } else {
            // parse 3x3 rule
            let b = p.0.as_bytes();
            let pattern = (b[0] == b'#') as usize
                | ((b[1] == b'#') as usize) << 1
                | ((b[2] == b'#') as usize) << 2
                | ((b[4] == b'#') as usize) << 3
                | ((b[5] == b'#') as usize) << 4
                | ((b[6] == b'#') as usize) << 5
                | ((b[8] == b'#') as usize) << 6
                | ((b[9] == b'#') as usize) << 7
                | ((b[10] == b'#') as usize) << 8;

            // parse 4x4 replacement pattern
            let r = p.1.as_bytes();
            let replacement = (r[0] == b'#') as usize
                | ((r[1] == b'#') as usize) << 1
                | ((r[2] == b'#') as usize) << 2
                | ((r[3] == b'#') as usize) << 3
                | ((r[5] == b'#') as usize) << 4
                | ((r[6] == b'#') as usize) << 5
                | ((r[7] == b'#') as usize) << 6
                | ((r[8] == b'#') as usize) << 7
                | ((r[10] == b'#') as usize) << 8
                | ((r[11] == b'#') as usize) << 9
                | ((r[12] == b'#') as usize) << 10
                | ((r[13] == b'#') as usize) << 11
                | ((r[15] == b'#') as usize) << 12
                | ((r[16] == b'#') as usize) << 13
                | ((r[17] == b'#') as usize) << 14
                | ((r[18] == b'#') as usize) << 15;

            // map all transformations of the rule to the replacement pattern
            for p in LUT_3[pattern] {
                if p == UM {
                    break;
                }
                rules_3x3[p as usize] = replacement;
            }
        }
    }

    let start_pattern = 0b_111_100_010;

    // Performance note: even though we're only storing results for 3x3 patterns
    // in the cache and even though it will just have about 100 entries in the
    // end, a HashMap is faster than a two-dimensional array
    let mut cache = FxHashMap::with_capacity_and_hasher(256, FxBuildHasher);

    let total2 = count(start_pattern, 3, 18, &rules_2x2, &rules_3x3, &mut cache);
    let total1 = count(start_pattern, 3, 5, &rules_2x2, &rules_3x3, &mut cache);

    println!("{total1}");
    println!("{total2}");
}
