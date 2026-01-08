//! Part 1 is a variant of [Langton's Ant]. Run this program with the
//! `visualize` feature enabled to see how the ant starts building a recurrent
//! "highway" (near the end of the visualization):
//!
//!     cargo run --release --features=visualize
//!
//! [Langton's Ant]: https://en.wikipedia.org/wiki/Langton%27s_ant
use std::fs;

#[cfg(feature = "visualize")]
use screen::WindowedScreen;

const SIZE: usize = 1024;

#[derive(Clone, Copy)]
enum State {
    Clean,
    Weakened,
    Infected,
    Flagged,
}

#[cfg(feature = "visualize")]
fn visualize(grid: &[State], virus: (usize, usize), screen: &mut WindowedScreen) {
    let mut map = grid
        .iter()
        .map(|s| match s {
            State::Clean => (' ', (0, 0, 0)),
            State::Weakened => ('⬮', (237, 167, 57)),
            State::Infected => ('⬮', (208, 58, 32)),
            State::Flagged => ('⬮', (112, 216, 215)),
        })
        .collect::<Vec<_>>();
    map[virus.1 * SIZE + virus.0] = ('█', (135, 247, 83));
    screen.update_with_colors(map, virus);
}

fn main() {
    let input = fs::read_to_string("input.txt").expect("Could not read file");
    let grid = input
        .lines()
        .map(|l| l.bytes().collect::<Vec<_>>())
        .collect::<Vec<_>>();

    for part1 in [true, false] {
        #[cfg(feature = "visualize")]
        let mut screen =
            WindowedScreen::new(SIZE, SIZE, if part1 { 400 } else { 800 }, Some((2, 2)));

        // a map of 1024 * 1024 should be more than large enough (for my input,
        // even 512x512 worked well)
        let mut map = vec![State::Clean; SIZE * SIZE];
        for (y, row) in grid.iter().enumerate() {
            for (x, cell) in row.iter().enumerate() {
                if *cell == b'#' {
                    map[(y + SIZE / 2) * SIZE + x + SIZE / 2] = State::Infected;
                }
            }
        }

        let mut dir = (0, -1);
        let mut virus = (
            (grid[0].len() / 2 + SIZE / 2) as usize,
            (grid.len() / 2 + SIZE / 2) as usize,
        );

        #[cfg(feature = "visualize")]
        visualize(&map, virus, &mut screen);

        let mut infects = 0;
        for _frame in 0..(if part1 { 10_000 } else { 10_000_000 }) {
            let s = &mut map[virus.1 * SIZE + virus.0];
            match s {
                State::Clean => {
                    dir = (dir.1, -dir.0);
                    *s = if part1 {
                        infects += 1;
                        State::Infected
                    } else {
                        State::Weakened
                    };
                }
                State::Weakened => {
                    infects += 1;
                    *s = State::Infected;
                }
                State::Infected => {
                    dir = (-dir.1, dir.0);
                    *s = if part1 { State::Clean } else { State::Flagged };
                }
                State::Flagged => {
                    dir = (-dir.0, -dir.1);
                    *s = State::Clean;
                }
            }
            virus.0 = virus.0.checked_add_signed(dir.0).unwrap();
            virus.1 = virus.1.checked_add_signed(dir.1).unwrap();

            #[cfg(feature = "visualize")]
            // Always visualize the entirety of part 1, but only frames 0-15000
            // and 9995000-10000000 of part 2. The visualization would be way
            // too long otherwise.
            if part1 || !(15000..=9995000).contains(&_frame) {
                visualize(&map, virus, &mut screen);
            }
        }

        #[cfg(feature = "visualize")]
        {
            screen.finish();
            println!();
        }

        println!("{infects}");
    }
}
