use std::{
    io::{Stdout, Write, stdout},
    thread,
    time::{Duration, Instant},
};

use crossterm::{
    ExecutableCommand, cursor,
    style::{self, Color, SetForegroundColor},
    terminal,
};

pub struct Screen {
    width: usize,
    height: usize,
    time_per_frame: Duration,
    last_grid: Vec<(char, Color)>,
    stdout: Stdout,
    pos: (u16, u16),
    last_update: Option<Instant>,
    finished: bool,
}

impl Screen {
    /// Create a new visualization with size `width * height` and the given
    /// frames per second
    pub fn new(width: usize, height: usize, fps: u32) -> Self {
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

        let last_grid = vec![(' ', Color::Grey); width * height];

        Self {
            width,
            height,
            time_per_frame: Duration::from_secs(1) / fps,
            last_grid,
            stdout,
            pos: cursor::position().unwrap(),
            last_update: None,
            finished: false,
        }
    }

    /// Update the visualization with a new grid
    pub fn update(&mut self, new_grid: &[char]) {
        if let Some(last_update) = self.last_update {
            let elapsed = last_update.elapsed();
            if self.time_per_frame > elapsed {
                thread::sleep(self.time_per_frame - elapsed);
            }
        }
        self.last_update = Some(Instant::now());

        let mut stdout = self.stdout.lock();
        for y in 0..self.height {
            for x in 0..self.width {
                let c = new_grid[y * self.width + x];
                if c != self.last_grid[y * self.width + x].0 {
                    self.last_grid[y * self.width + x].0 = c;
                    stdout
                        .execute(cursor::MoveTo(self.pos.0 + x as u16, self.pos.1 + y as u16))
                        .unwrap();
                    stdout.execute(style::Print(c)).unwrap();
                }
            }
        }
    }

    /// Update the visualization with a new colored grid
    pub fn update_with_colors(&mut self, new_grid: &[(char, (u8, u8, u8))]) {
        if let Some(last_update) = self.last_update {
            let elapsed = last_update.elapsed();
            if self.time_per_frame > elapsed {
                thread::sleep(self.time_per_frame - elapsed);
            }
        }
        self.last_update = Some(Instant::now());

        let mut stdout = self.stdout.lock();
        for y in 0..self.height {
            for x in 0..self.width {
                let (r, g, b) = new_grid[y * self.width + x].1;
                let c = (new_grid[y * self.width + x].0, Color::Rgb { r, g, b });
                if c != self.last_grid[y * self.width + x] {
                    self.last_grid[y * self.width + x] = c;
                    stdout.execute(SetForegroundColor(c.1)).unwrap();
                    stdout
                        .execute(cursor::MoveTo(self.pos.0 + x as u16, self.pos.1 + y as u16))
                        .unwrap();
                    stdout.execute(style::Print(c.0)).unwrap();
                    stdout.execute(SetForegroundColor(Color::Grey)).unwrap();
                }
            }
        }
    }

    /// Finish visualization and reset terminal
    pub fn finish(&mut self) {
        if self.finished {
            return;
        }
        let mut stdout = self.stdout.lock();
        stdout
            .execute(cursor::MoveTo(0, self.pos.1 + self.height as u16 + 1))
            .unwrap();
        stdout.execute(cursor::Show).unwrap();
        self.finished = true;
    }
}

impl Drop for Screen {
    fn drop(&mut self) {
        self.finish();
    }
}
