use core::time;
use std::{
    io::{stdout, Stdout},
    thread,
};

use crossterm::{cursor, style, ExecutableCommand};

pub struct Screen {
    width: usize,
    height: usize,
    last_grid: Vec<char>,
    stdout: Stdout,
    pos: (u16, u16),
}

impl Screen {
    pub fn new(width: usize, height: usize) -> Self {
        let mut stdout = stdout();

        // make space on screen and reset cursor
        for _ in 0..height {
            println!();
        }
        stdout
            .execute(cursor::MoveTo(
                0,
                cursor::position().unwrap().1 - height as u16,
            ))
            .unwrap();

        // hide cursor
        stdout.execute(cursor::Hide).unwrap();

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
                    self.stdout
                        .execute(cursor::MoveTo(self.pos.0 + x as u16, self.pos.1 + y as u16))
                        .unwrap();
                    self.stdout.execute(style::Print(c)).unwrap();
                }
            }
        }
        thread::sleep(time::Duration::from_millis(5));
    }

    pub fn finish(&mut self) {
        self.stdout
            .execute(cursor::MoveTo(0, self.pos.1 + self.height as u16 + 1))
            .unwrap();
        self.stdout.execute(cursor::Show).unwrap();
    }
}
