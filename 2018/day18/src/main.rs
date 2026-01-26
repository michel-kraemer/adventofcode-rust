use std::fs;

use rustc_hash::FxHashMap;

const HALF: usize = 31;
const OPEN: u64 = 0b00;
const TREES: u64 = 0b01;
const LUMBERYARDS: u64 = 0b10;

const MASK_T: u64 = 0b11;
const MASK_TTT: u64 = 0b111111;
const MASK_TFT: u64 = 0b110011;

const MASK_TREES: u64 = TREES << 4 | TREES << 2 | TREES;
const MASK_LUMBERYARDS: u64 = LUMBERYARDS << 4 | LUMBERYARDS << 2 | LUMBERYARDS;

fn update(a: u64, b: u64, c: u64, g: u64, ng: &mut u64, sh: usize) {
    let trees = (a & MASK_TREES).count_ones()
        + (b & MASK_TREES).count_ones()
        + (c & MASK_TREES).count_ones();
    let lumberyards = (a & MASK_LUMBERYARDS).count_ones()
        + (b & MASK_LUMBERYARDS).count_ones()
        + (c & MASK_LUMBERYARDS).count_ones();

    let c = (g >> sh) & MASK_T;
    if c == OPEN && trees >= 3 {
        *ng |= TREES << sh;
    } else if c == TREES {
        if lumberyards >= 3 {
            *ng |= LUMBERYARDS << sh;
        } else {
            *ng |= TREES << sh;
        }
    } else if c == LUMBERYARDS && lumberyards > 0 && trees > 0 {
        *ng |= LUMBERYARDS << sh;
    }
}

fn main() {
    let input = fs::read_to_string("input.txt").expect("Could not read file");
    let mut width = 0;
    let mut grid = input
        .lines()
        .map(|l| {
            // Convert acres into a one-hot encoded bit mask where 2 bits
            // represent one acre. The input grid is 50 acres wide, so we need
            // 100 bits (= two 64-bit words). We leave the first pair of bits
            // in each word empty to make it easier to extract acres later.
            let mut word1: u64 = 0;
            let mut word2: u64 = 0;
            width = l.len();
            for (i, b) in l.bytes().enumerate() {
                let bits = match b {
                    b'|' => TREES,
                    b'#' => LUMBERYARDS,
                    _ => OPEN,
                };
                if i < HALF {
                    word1 |= bits << (i * 2 + 2);
                } else {
                    word2 |= bits << ((i - HALF) * 2 + 2);
                }
            }
            (word1, word2)
        })
        .collect::<Vec<_>>();

    grid.insert(0, (0, 0));
    grid.push((0, 0));

    for part1 in [true, false] {
        let mut grid = grid.clone();
        let max_steps = if part1 { 10 } else { 1_000_000_000 };

        let mut seen = FxHashMap::default();
        seen.insert(grid.clone(), 0);

        let mut new_grid = vec![(0, 0); grid.len()];
        let mut step = 0;
        while step < max_steps {
            new_grid.fill((0, 0));

            for (y, g) in grid.windows(3).enumerate() {
                // process pairs of bits in first word (except for the last
                // pair)
                for x in 1..HALF {
                    let x = x * 2;
                    let (a, b, c) = (
                        (g[0].0 >> (x - 2)) & MASK_TTT,
                        (g[1].0 >> (x - 2)) & MASK_TFT,
                        (g[2].0 >> (x - 2)) & MASK_TTT,
                    );
                    update(a, b, c, g[1].0, &mut new_grid[y + 1].0, x);
                }

                // process last pair of bits in first word - also count first
                // pair of bits of second word
                let (mut a, mut b, mut c) = (
                    (g[0].0 >> ((HALF - 1) * 2)) & MASK_TTT,
                    (g[1].0 >> ((HALF - 1) * 2)) & MASK_TFT,
                    (g[2].0 >> ((HALF - 1) * 2)) & MASK_TTT,
                );
                a |= ((g[0].1 >> 2) & MASK_T) << 4;
                b |= ((g[1].1 >> 2) & MASK_T) << 4;
                c |= ((g[2].1 >> 2) & MASK_T) << 4;
                update(a, b, c, g[1].0, &mut new_grid[y + 1].0, HALF * 2);

                // process first pair of bits in second pair - also count last
                // pair of bits of first word
                let (mut a, mut b, mut c) =
                    (g[0].1 & MASK_TTT, g[1].1 & MASK_TFT, g[2].1 & MASK_TTT);
                a |= g[0].0 >> (HALF * 2);
                b |= g[1].0 >> (HALF * 2);
                c |= g[2].0 >> (HALF * 2);
                update(a, b, c, g[1].1, &mut new_grid[y + 1].1, 2);

                // process pairs of bits in second word (except for the first
                // pair)
                for x in HALF + 2..=width {
                    let x = (x - HALF) * 2;
                    let (a, b, c) = (
                        (g[0].1 >> (x - 2)) & MASK_TTT,
                        (g[1].1 >> (x - 2)) & MASK_TFT,
                        (g[2].1 >> (x - 2)) & MASK_TTT,
                    );
                    update(a, b, c, g[1].1, &mut new_grid[y + 1].1, x);
                }
            }

            (grid, new_grid) = (new_grid, grid);
            step += 1;

            // find cycle and skip ahead if possible
            if let Some(start) = seen.get(&grid) {
                step = max_steps - ((max_steps - step) % (step - start));
                seen.clear();
            } else {
                seen.insert(grid.clone(), step);
            }
        }

        // count trees and lumberyards
        let mut trees = 0;
        let mut lumberyards = 0;
        for (word1, word2) in grid.iter().skip(1).take(grid.len() - 2) {
            for (w, l) in [(word1, HALF), (word2, width - HALF)] {
                let mut i = 2;
                while i < (l + 1) * 2 {
                    let bits = (w >> i) & MASK_T;
                    match bits {
                        TREES => trees += 1,
                        LUMBERYARDS => lumberyards += 1,
                        _ => {}
                    }
                    i += 2;
                }
            }
        }

        println!("{}", trees * lumberyards);
    }
}
