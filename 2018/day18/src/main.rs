use std::fs;

use rustc_hash::{FxBuildHasher, FxHashMap};
#[cfg(feature = "visualize")]
use screen::Screen;

const OPEN: u8 = 0b00;
const TREES: u8 = 0b01;
const LUMBERYARD: u8 = 0b10;

const MASK_TREES: u8 = TREES << 4 | TREES << 2 | TREES;
const MASK_LUMBERYARD: u8 = LUMBERYARD << 4 | LUMBERYARD << 2 | LUMBERYARD;

#[cfg(feature = "visualize")]
fn visualize(grid: &[u8], width: usize, height: usize, screen: &mut Screen) {
    let mut new_grid = vec![(' ', (0, 0, 0)); (width - 2) * height];
    for y in 0..height {
        for x in 1..width - 1 {
            new_grid[y * (width - 2) + x - 1] = match grid[y * width + x] {
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
    let mut width = 0;
    let mut height = 0;
    let mut grid = input
        .lines()
        .flat_map(|l| {
            width = l.len() + 2;
            height += 1;
            // add empty cells at the beginning and at the end to make it easier
            // to count neighbors
            std::iter::once(b'.')
                .chain(l.bytes())
                .chain(std::iter::once(b'.'))
                .map(|b| match b {
                    b'.' => OPEN,
                    b'|' => TREES,
                    _ => LUMBERYARD,
                })
        })
        .collect::<Vec<_>>();

    // add an empty row at the end to make it easier to count neighbors
    grid.resize(grid.len() + width, OPEN);

    for part1 in [true, false] {
        let mut grid = grid.clone();
        let max_steps = if part1 { 10 } else { 1_000_000_000 };

        #[cfg(feature = "visualize")]
        let mut screen = if !part1 {
            let mut screen = Screen::new(width - 2, height, 20);
            visualize(&grid, width, height, &mut screen);
            Some(screen)
        } else {
            None
        };

        let mut seen = FxHashMap::with_capacity_and_hasher(1000, FxBuildHasher);
        seen.insert(grid.clone(), 0);

        let mut row = vec![0; width];
        let mut step = 0;
        while step < max_steps {
            row.copy_from_slice(&grid[0..width]);

            for y in 0..height {
                row[1] = (row[1] << 2) | grid[(y + 1) * width + 1];

                for x in 1..row.len() - 1 {
                    row[x + 1] = (row[x + 1] << 2) | grid[(y + 1) * width + x + 1];

                    // count trees and lumberyards in a 3x3 area
                    let trees = (row[x - 1] & MASK_TREES).count_ones()
                        + (row[x] & MASK_TREES).count_ones()
                        + (row[x + 1] & MASK_TREES).count_ones();
                    let lumberyards = (row[x - 1] & MASK_LUMBERYARD).count_ones()
                        + (row[x] & MASK_LUMBERYARD).count_ones()
                        + (row[x + 1] & MASK_LUMBERYARD).count_ones();

                    let gc = &mut grid[y * width + x];
                    *gc = if *gc == OPEN {
                        if trees >= 3 { TREES } else { OPEN }
                    } else if *gc == TREES {
                        if lumberyards >= 3 { LUMBERYARD } else { TREES }
                    } else if trees == 0 || lumberyards == 1 {
                        // we are comparing `lumberyards` to 1 and not 0,
                        // because the count includes the cell in the center
                        OPEN
                    } else {
                        LUMBERYARD
                    };
                }
            }

            step += 1;

            // find cycle and skip ahead if possible
            if let Some(start) = seen.insert(grid.clone(), step) {
                step = max_steps - ((max_steps - step) % (step - start));
                seen.clear();
            }

            #[cfg(feature = "visualize")]
            if let Some(screen) = &mut screen {
                visualize(&grid, width, height, screen);
            }
        }

        #[cfg(feature = "visualize")]
        drop(screen);

        // count trees and lumberyards
        let mut trees = 0;
        let mut lumberyards = 0;
        for &b in grid.iter().take(grid.len() - width) {
            match b {
                TREES => trees += 1,
                LUMBERYARD => lumberyards += 1,
                _ => {}
            }
        }

        println!("{}", trees * lumberyards);
    }
}
