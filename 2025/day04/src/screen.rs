use core::time;
use std::{
    io::{Stdout, Write, stdout},
    thread,
};

use crossterm::{ExecutableCommand, cursor, style, terminal};

pub struct Screen {
    width: usize,
    height: usize,
    last_grid: Vec<char>,
    stdout: Stdout,
    pos: (u16, u16),
}

impl Screen {
    pub fn new(width: usize, height: usize) -> Self {
        let hh = height.div_ceil(2);

        let (terminal_cols, terminal_rows) = terminal::size().unwrap();
        if (terminal_rows as usize) < hh {
            panic!(
                "Terminal window is not high enough. Resize your terminal \
                window (or zoom out), so at least {hh} rows can be displayed \
                at once. Your terminal has a height of {terminal_rows} rows."
            );
        }
        if (terminal_cols as usize) < width {
            panic!(
                "Terminal window is not wide enough. Resize your terminal \
                window (or zoom out), so at least {width} columns can be displayed \
                at once. Your terminal has a width of {terminal_cols} columns."
            );
        }

        let stdout = stdout();
        let mut lock = stdout.lock();

        // make space on screen and reset cursor
        for _ in 0..hh {
            lock.write_all(b"\n").unwrap();
        }
        lock.execute(cursor::MoveTo(0, cursor::position().unwrap().1 - hh as u16))
            .unwrap();

        // hide cursor
        lock.execute(cursor::Hide).unwrap();

        let last_grid = vec![' '; width * hh];

        Self {
            width,
            height,
            last_grid,
            stdout,
            pos: cursor::position().unwrap(),
        }
    }

    pub fn update(&mut self, grid: &[u8]) {
        let mut stdout = self.stdout.lock();
        for y in (0..self.height).step_by(2) {
            for x in 0..self.width {
                let t = grid[y * self.width + x] > 0;
                let b = if y < self.height - 1 {
                    grid[(y + 1) * self.width + x] > 0
                } else {
                    false
                };
                let c = match (t, b) {
                    (true, true) => '█',
                    (true, false) => '▀',
                    (false, true) => '▄',
                    (false, false) => ' ',
                };
                if c != self.last_grid[y / 2 * self.width + x] {
                    self.last_grid[y / 2 * self.width + x] = c;
                    stdout
                        .execute(cursor::MoveTo(
                            self.pos.0 + x as u16,
                            self.pos.1 + (y / 2) as u16,
                        ))
                        .unwrap();
                    stdout.execute(style::Print(c)).unwrap();
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
