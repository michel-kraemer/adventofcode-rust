use std::fs;

use rustc_hash::FxHashMap;
#[cfg(feature = "visualize")]
use screen::Screen;

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

    let contents = (g >> sh) & MASK_T;
    if contents == OPEN && trees >= 3 {
        *ng |= TREES << sh;
    } else if contents == TREES {
        if lumberyards >= 3 {
            *ng |= LUMBERYARDS << sh;
        } else {
            *ng |= TREES << sh;
        }
    } else if contents == LUMBERYARDS && lumberyards > 0 && trees > 0 {
        *ng |= LUMBERYARDS << sh;
    }
}

#[cfg(feature = "visualize")]
fn visualize(grid: &[(u64, u64)], width: usize, half: usize, screen: &mut Screen) {
    let mut new_grid = vec![(' ', (0, 0, 0)); width * grid.len() - 2];
    for (row, (word1, word2)) in grid.iter().skip(1).take(grid.len() - 2).enumerate() {
        let mut col = 0;
        for (w, l) in [(word1, half), (word2, width - half)] {
            let mut i = 2;
            while i < (l + 1) * 2 {
                let bits = (w >> i) & MASK_T;
                new_grid[row * width + col] = match bits {
                    TREES => ('█', (14, 200, 0)),
                    LUMBERYARDS => ('▒', (9, 120, 0)),
                    _ => ('░', (5, 65, 0)),
                };
                i += 2;
                col += 1;
            }
        }
    }
    screen.update_with_colors(new_grid);
}

fn main() {
    let input = fs::read_to_string("input.txt").expect("Could not read file");
    let mut width = 0;
    let mut half = 0;
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
            assert!(
                width <= 62,
                "This solution only works for grids with a maximum width of 62 bytes"
            );
            half = width / 2;
            for (i, b) in l.bytes().enumerate() {
                let bits = match b {
                    b'|' => TREES,
                    b'#' => LUMBERYARDS,
                    _ => OPEN,
                };
                if i < half {
                    word1 |= bits << (i * 2 + 2);
                } else {
                    word2 |= bits << ((i - half) * 2 + 2);
                }
            }
            (word1, word2)
        })
        .collect::<Vec<_>>();

    // add empty rows at the beginning and the end to make it easier to count
    // bits
    grid.insert(0, (0, 0));
    grid.push((0, 0));

    for part1 in [true, false] {
        let mut grid = grid.clone();
        let max_steps = if part1 { 10 } else { 1_000_000_000 };

        #[cfg(feature = "visualize")]
        let mut screen = if !part1 {
            let mut screen = Screen::new(width, grid.len() - 2, 20);
            visualize(&grid, width, half, &mut screen);
            Some(screen)
        } else {
            None
        };

        let mut seen = FxHashMap::default();
        seen.insert(grid.clone(), 0);

        let mut new_grid = vec![(0, 0); grid.len()];
        let mut step = 0;
        while step < max_steps {
            new_grid.fill((0, 0));

            for (y, g) in grid.windows(3).enumerate() {
                let prev1 = g[0].0;
                let curr1 = g[1].0;
                let next1 = g[2].0;

                let prev2 = g[0].1;
                let curr2 = g[1].1;
                let next2 = g[2].1;

                let mut ng1 = 0;
                let mut ng2 = 0;

                // process pairs of bits in first word (except for the last
                // pair)
                for x in 1..half {
                    let x = x * 2;
                    let (a, b, c) = (
                        (prev1 >> (x - 2)) & MASK_TTT,
                        (curr1 >> (x - 2)) & MASK_TFT,
                        (next1 >> (x - 2)) & MASK_TTT,
                    );
                    update(a, b, c, curr1, &mut ng1, x);
                }

                // process last pair of bits in first word - also count first
                // pair of bits of second word
                let (mut a, mut b, mut c) = (
                    (prev1 >> ((half - 1) * 2)) & MASK_TTT,
                    (curr1 >> ((half - 1) * 2)) & MASK_TFT,
                    (next1 >> ((half - 1) * 2)) & MASK_TTT,
                );
                a |= ((prev2 >> 2) & MASK_T) << 4;
                b |= ((curr2 >> 2) & MASK_T) << 4;
                c |= ((next2 >> 2) & MASK_T) << 4;
                update(a, b, c, curr1, &mut ng1, half * 2);

                // process first pair of bits in second pair - also count last
                // pair of bits of first word
                let (mut a, mut b, mut c) = (prev2 & MASK_TTT, curr2 & MASK_TFT, next2 & MASK_TTT);
                a |= prev1 >> (half * 2);
                b |= curr1 >> (half * 2);
                c |= next1 >> (half * 2);
                update(a, b, c, curr2, &mut ng2, 2);

                // process pairs of bits in second word (except for the first
                // pair)
                for x in half + 2..=width {
                    let x = (x - half) * 2;
                    let (a, b, c) = (
                        (prev2 >> (x - 2)) & MASK_TTT,
                        (curr2 >> (x - 2)) & MASK_TFT,
                        (next2 >> (x - 2)) & MASK_TTT,
                    );
                    update(a, b, c, curr2, &mut ng2, x);
                }

                new_grid[y + 1].0 = ng1;
                new_grid[y + 1].1 = ng2;
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

            #[cfg(feature = "visualize")]
            if let Some(screen) = &mut screen {
                visualize(&grid, width, half, screen);
            }
        }

        #[cfg(feature = "visualize")]
        drop(screen);

        // count trees and lumberyards
        let mut trees = 0;
        let mut lumberyards = 0;
        for (word1, word2) in grid.iter().skip(1).take(grid.len() - 2) {
            for (w, l) in [(word1, half), (word2, width - half)] {
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
