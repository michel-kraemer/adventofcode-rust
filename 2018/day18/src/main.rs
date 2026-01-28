use std::fs;

use rustc_hash::{FxBuildHasher, FxHashMap};
#[cfg(feature = "visualize")]
use screen::Screen;

const OPEN: u8 = 0b00;
const TREES: u8 = 0b01;
const LUMBERYARD: u8 = 0b10;

const MASK_TREES: u8 = TREES << 4 | TREES << 2 | TREES;
const MASK_TREES_CURR: u8 = TREES << 4 | TREES;
const MASK_LUMBERYARD: u8 = LUMBERYARD << 4 | LUMBERYARD << 2 | LUMBERYARD;
const MASK_LUMBERYARD_CURR: u8 = LUMBERYARD << 4 | LUMBERYARD;

#[cfg(feature = "visualize")]
fn visualize(grid: &[Vec<u8>], screen: &mut Screen) {
    let width = grid[0].len() - 2;
    let mut new_grid = vec![(' ', (0, 0, 0)); width * grid.len() - 2];
    for (y, row) in grid.iter().skip(1).enumerate() {
        for (x, b) in row.iter().skip(1).take(width).enumerate() {
            new_grid[y * width + x] = match *b {
                TREES => ('█', (14, 200, 0)),
                LUMBERYARD => ('▒', (9, 120, 0)),
                _ => ('░', (5, 65, 0)),
            };
        }
    }
    screen.update_with_colors(new_grid);
}

fn main() {
    let input = fs::read_to_string("input.txt").expect("Could not read file");
    let mut grid = input
        .lines()
        .map(|l| {
            std::iter::once(b'.')
                .chain(l.bytes())
                .chain(std::iter::once(b'.'))
                .map(|b| match b {
                    b'.' => OPEN,
                    b'|' => TREES,
                    _ => LUMBERYARD,
                })
                .collect::<Vec<_>>()
        })
        .collect::<Vec<_>>();

    // add empty rows at the beginning and the end to make it easier to count
    // neighbors
    grid.insert(0, vec![OPEN; grid[0].len()]);
    grid.push(vec![OPEN; grid[0].len()]);

    for part1 in [true, false] {
        let mut grid = grid.clone();
        let max_steps = if part1 { 10 } else { 1_000_000_000 };

        #[cfg(feature = "visualize")]
        let mut screen = if !part1 {
            let mut screen = Screen::new(grid[0].len() - 2, grid.len() - 2, 20);
            visualize(&grid, &mut screen);
            Some(screen)
        } else {
            None
        };

        let mut seen = FxHashMap::with_capacity_and_hasher(1000, FxBuildHasher);
        seen.insert(grid.clone(), 0);

        let mut new_grid = grid.clone();
        let mut step = 0;
        while step < max_steps {
            for (g, ng) in grid.windows(3).zip(new_grid.iter_mut().skip(1)) {
                let mut prev = g[0][0] << 2 | g[0][1];
                let mut curr = g[1][0] << 2 | g[1][1];
                let mut next = g[2][0] << 2 | g[2][1];

                let mut i = 0;
                while i < g[0].len() - 2 {
                    prev = (prev << 2) | g[0][i + 2];
                    curr = (curr << 2) | g[1][i + 2];
                    next = (next << 2) | g[2][i + 2];

                    let trees = (prev & MASK_TREES).count_ones()
                        + (curr & MASK_TREES_CURR).count_ones()
                        + (next & MASK_TREES).count_ones();
                    let lumberyards = (prev & MASK_LUMBERYARD).count_ones()
                        + (curr & MASK_LUMBERYARD_CURR).count_ones()
                        + (next & MASK_LUMBERYARD).count_ones();

                    let gc = g[1][i + 1];
                    ng[i + 1] = if gc == OPEN {
                        if trees >= 3 { TREES } else { OPEN }
                    } else if gc == TREES {
                        if lumberyards >= 3 { LUMBERYARD } else { TREES }
                    } else if lumberyards == 0 || trees == 0 {
                        OPEN
                    } else {
                        LUMBERYARD
                    };

                    i += 1;
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

            #[cfg(feature = "visualize")]
            if let Some(screen) = &mut screen {
                visualize(&grid, screen);
            }
        }

        #[cfg(feature = "visualize")]
        drop(screen);

        // count trees and lumberyards
        let mut trees = 0;
        let mut lumberyards = 0;
        for &b in grid.iter().flatten() {
            match b {
                TREES => trees += 1,
                LUMBERYARD => lumberyards += 1,
                _ => {}
            }
        }

        println!("{}", trees * lumberyards);
    }
}
