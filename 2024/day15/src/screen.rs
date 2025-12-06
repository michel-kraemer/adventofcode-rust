use core::time;
use std::{
    io::{stdout, Stdout, Write},
    thread,
};

use crossterm::{cursor, style, terminal, ExecutableCommand};

pub struct Screen {
    width: usize,
    height: usize,
    last_grid: Vec<char>,
    stdout: Stdout,
    pos: (u16, u16),
}

impl Screen {
    pub fn new(width: usize, height: usize) -> Self {
        let (terminal_cols, terminal_rows) = terminal::size().unwrap();
        if (terminal_rows as usize) < height {
            panic!(
                "Terminal window is not high enough. Resize your terminal \
                window, so at least {height} rows can be displayed at once."
            );
        }
        if (terminal_cols as usize) < height {
            panic!(
                "Terminal window is not wide enough. Resize your terminal \
                window, so at least {width} columns can be displayed at once."
            );
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

        let last_grid = vec![' '; width * height];

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
        for y in 0..self.height {
            for x in 0..self.width {
                let mut c = grid[y * self.width + x] as char;
                if c == '.' {
                    c = ' ';
                } else if c == '#' {
                    c = '█';
                }
                if c != self.last_grid[y * self.width + x] {
                    self.last_grid[y * self.width + x] = c;
                    stdout
                        .execute(cursor::MoveTo(self.pos.0 + x as u16, self.pos.1 + y as u16))
                        .unwrap();
                    stdout.execute(style::Print(c)).unwrap();
                }
            }
        }
        thread::sleep(time::Duration::from_millis(5));
    }

    pub fn finish(&mut self) {
        let mut stdout = self.stdout.lock();
        stdout
            .execute(cursor::MoveTo(0, self.pos.1 + self.height as u16 + 1))
            .unwrap();
        stdout.execute(cursor::Show).unwrap();
    }
}
