use std::fs;

#[cfg(feature = "visualize")]
use screen::Screen;

/// Rows and columns
const H: usize = 100;

/// Bits per cell
const BPC: usize = 4;

/// Cells per word
const CPW: usize = 64 / BPC;

/// Words per row
const W: usize = H.div_ceil(CPW);

/// The number of iterations to perform
const ITERATIONS: usize = 100;

fn set_corner_bits(grid: &mut [u64; (H + 2) * W]) {
    grid[W] |= 1;
    grid[H * W] |= 1;
    grid[W + W - 1] |= 1 << ((H % CPW - 1) * BPC);
    grid[H * W + W - 1] |= 1 << ((H % CPW - 1) * BPC);
}

/// Run game of life for a given number of [ITERATIONS]. This implementation is
/// based on the algorithm described in Rokicki's paper Life Algorithms, Section
/// 2.4 "Single Instruction Multiple Data" \[1\], which is part of
/// Gathering4Gardner's G4G13 Gift Exchange Book \[2\].
///
/// \[1\] Tomas Rokicki (2018). Life Algorithms.
///     https://www.gathering4gardner.org/g4g13gift/math/RokickiTomas-GiftExchange-LifeAlgorithms-G4G13.pdf\
/// \[2\] https://www.gathering4gardner.org/g4g13-exchange-book/
fn run<'a>(
    mut grid: &'a mut [u64; (H + 2) * W],
    mut new_grid: &'a mut [u64; (H + 2) * W],
    keep_corners: bool,
) -> u32 {
    #[cfg(feature = "visualize")]
    let mut screen = Screen::new(H, H / 2, 15);

    if keep_corners {
        set_corner_bits(grid);
    }

    #[cfg(feature = "visualize")]
    visualize(grid, &mut screen);

    for _ in 0..ITERATIONS {
        for y in 1..=H {
            let cy = y * W;
            let py = cy - W;
            let ny = cy + W;

            for w in 0..W {
                let pw = grid[py + w];
                let cw = grid[cy + w];
                let nw = grid[ny + w];
                let mut n = (pw << BPC)
                    + pw
                    + (pw >> BPC)
                    + (cw << BPC)
                    + (cw >> BPC)
                    + (nw << BPC)
                    + nw
                    + (nw >> BPC);
                if w > 0 {
                    n += (grid[py + w - 1] + grid[cy + w - 1] + grid[ny + w - 1]) >> 60;
                }
                if w < W - 1 {
                    n += (grid[py + w + 1] + grid[cy + w + 1] + grid[ny + w + 1]) << 60;
                }
                let ng = n | cw;
                new_grid[cy + w] = ng & (ng >> 1) & (!((ng >> 2) | (ng >> 3))) & 0x1111111111111111;
            }

            // mask out bits not part of the HxH grid
            new_grid[cy + W - 1] &= (1 << ((H % CPW) * BPC)) - 1;
        }

        if keep_corners {
            set_corner_bits(new_grid);
        }

        (grid, new_grid) = (new_grid, grid);

        #[cfg(feature = "visualize")]
        visualize(grid, &mut screen);
    }

    grid.iter().map(|w| w.count_ones()).sum::<u32>()
}

#[cfg(feature = "visualize")]
fn visualize(grid: &mut [u64; (H + 2) * W], screen: &mut Screen) {
    let mut screen_grid = vec!['.'; H / 2 * H];
    for y in 0..H / 2 {
        for x in 0..H {
            let mask: u64 = 1 << ((x % CPW) * BPC);
            let t = grid[y * 2 * W + x / CPW] & mask > 0;
            let b = grid[(y * 2 + 1) * W + x / CPW] & mask > 0;
            let c = match (t, b) {
                (true, true) => '█',
                (true, false) => '▀',
                (false, true) => '▄',
                (false, false) => ' ',
            };
            screen_grid[y * H + x] = c;
        }
    }
    screen.update(&screen_grid);
}

fn main() {
    let input = fs::read_to_string("input.txt").expect("Could not read file");

    // add an empty row at the top and one at the bottom
    let mut grid: [u64; (H + 2) * W] = [0; (H + 2) * W];
    let mut new_grid: [u64; (H + 2) * W] = [0; (H + 2) * W];

    let mut i = W;
    for l in input.lines() {
        for (j, b) in l.bytes().enumerate() {
            if b == b'.' {
                continue;
            }
            grid[i + j / CPW] |= 1 << ((j % CPW) * BPC);
        }
        i += W;
    }

    let mut orig_grid = grid;
    println!("{}", run(&mut grid, &mut new_grid, false));
    println!("{}", run(&mut orig_grid, &mut new_grid, true));
}
