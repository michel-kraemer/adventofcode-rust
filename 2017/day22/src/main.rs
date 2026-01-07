use std::fs;

#[derive(Clone, Copy)]
enum State {
    Clean,
    Weakened,
    Infected,
    Flagged,
}

fn main() {
    let input = fs::read_to_string("input.txt").expect("Could not read file");
    let grid = input
        .lines()
        .map(|l| l.bytes().collect::<Vec<_>>())
        .collect::<Vec<_>>();

    for part1 in [true, false] {
        // a map of 1024 * 1024 should be more than large enough (for my input,
        // even 512x512 worked well)
        let mut map = vec![State::Clean; 1024 * 1024];
        for (y, row) in grid.iter().enumerate() {
            for (x, cell) in row.iter().enumerate() {
                if *cell == b'#' {
                    map[(y + 512) * 1024 + x + 512] = State::Infected;
                }
            }
        }

        let mut dir = (0, -1);
        let mut virus = (
            (grid[0].len() / 2 + 512) as usize,
            (grid.len() / 2 + 512) as usize,
        );

        let mut infects = 0;
        for _ in 0..(if part1 { 10_000 } else { 10_000_000 }) {
            let s = &mut map[virus.1 * 1024 + virus.0];
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
        }

        println!("{infects}");
    }
}
