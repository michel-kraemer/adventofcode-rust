use core::time;
use std::{
    io::{Stdout, Write, stdout},
    thread,
};

use crossterm::{
    ExecutableCommand, cursor,
    style::{self, Color, SetForegroundColor},
    terminal,
};
use rand::{rng, seq::SliceRandom};
use scarlet::{
    color::RGBColor,
    colormap::{ColorMap, ListedColorMap},
};

pub struct Screen {
    height: usize,
    last_grid: Vec<Vec<u64>>,
    stdout: Stdout,
    pos: (u16, u16),
    colors: Vec<RGBColor>,
}

impl Screen {
    pub fn new(width: usize, height: usize) -> Self {
        let (terminal_cols, terminal_rows) = terminal::size().unwrap();
        if (terminal_rows as usize) < height {
            eprintln!(
                "Terminal window is not high enough. Resize your terminal \
                window (or zoom out), so at least {height} rows can be displayed \
                at once. Your terminal has a height of {terminal_rows} rows."
            );
            std::process::exit(1);
        }
        if (terminal_cols as usize) < width {
            eprintln!(
                "Terminal window is not wide enough. Resize your terminal \
                window (or zoom out), so at least {width} columns can be displayed \
                at once. Your terminal has a width of {terminal_cols} columns."
            );
            std::process::exit(1);
        }

        let stdout = stdout();
        let mut lock = stdout.lock();

        // make space on screen and reset cursor
        for _ in 0..height {
            lock.write_all(b"\n").unwrap();
        }
        lock.execute(cursor::MoveTo(
            0,
            cursor::position().unwrap().1 - height as u16,
        ))
        .unwrap();

        // hide cursor
        lock.execute(cursor::Hide).unwrap();

        let last_grid = vec![vec![u64::MAX; width]; height];

        let color_map = ListedColorMap::turbo();
        let mut steps = Vec::new();
        let n_colors = 220;
        for s in 1..=220 {
            steps.push((1.0 / n_colors as f64) * s as f64);
        }
        steps.drain(1..20);
        let mut rng = rng();
        steps.shuffle(&mut rng);
        let colors: Vec<RGBColor> = color_map.transform(steps);

        Self {
            height,
            last_grid,
            stdout,
            pos: cursor::position().unwrap(),
            colors,
        }
    }

    pub fn update(&mut self, grid: &[Vec<u64>]) {
        let mut stdout = self.stdout.lock();
        for (y, row) in grid.iter().enumerate() {
            for (x, &i) in row.iter().enumerate() {
                if i != self.last_grid[y][x] {
                    self.last_grid[y][x] = i;
                    stdout
                        .execute(cursor::MoveTo(self.pos.0 + x as u16, self.pos.1 + y as u16))
                        .unwrap();
                    if i == 0 {
                        stdout.execute(style::Print('.')).unwrap();
                    } else {
                        let col = self.colors[(i as usize) % self.colors.len()];
                        stdout
                            .execute(SetForegroundColor(Color::Rgb {
                                r: col.int_r(),
                                g: col.int_g(),
                                b: col.int_b(),
                            }))
                            .unwrap();
                        stdout
                            .execute(style::Print(if i == 0 { '.' } else { '#' }))
                            .unwrap();
                        stdout.execute(SetForegroundColor(Color::Grey)).unwrap();
                    }
                }
            }
        }
        thread::sleep(time::Duration::from_millis(2));
    }

    pub fn finish(&mut self) {
        let mut stdout = self.stdout.lock();
        stdout
            .execute(cursor::MoveTo(0, self.pos.1 + self.height as u16 + 1))
            .unwrap();
        stdout.execute(cursor::Show).unwrap();
    }
}
