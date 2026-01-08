use std::{
    io::{Stdout, Write, stdout},
    thread::{self, JoinHandle},
    time::Duration,
};

use crossbeam_channel::{Sender, bounded};
use crossterm::{ExecutableCommand, cursor, terminal};

use crate::renderer::{RenderMessage, Renderer};

mod renderer;

pub struct Screen {
    width: usize,
    height: usize,
    stdout: Stdout,
    pos: (u16, u16),
    thread_handle: Option<JoinHandle<()>>,
    sender: Option<Sender<RenderMessage>>,
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
        let pos = cursor::position().unwrap();

        // hide cursor
        lock.execute(cursor::Hide).unwrap();

        // start render loop
        let (sender, receiver) = bounded::<RenderMessage>(10);
        let thread_handle = thread::spawn(move || {
            let mut thread = Renderer::new(width, height, Duration::from_secs(1) / fps, pos);
            for msg in &receiver {
                thread.render(msg, receiver.len());
            }
        });

        Self {
            width,
            height,
            stdout,
            pos,
            thread_handle: Some(thread_handle),
            sender: Some(sender),
            finished: false,
        }
    }

    /// Update the visualization with a new grid
    pub fn update(&mut self, new_grid: Vec<char>) {
        if let Some(sender) = &mut self.sender {
            sender
                .send(RenderMessage::Render { new_grid })
                .expect("Render channel is closed");
        }
    }

    /// Update the visualization with a new colored grid
    pub fn update_with_colors(&mut self, new_grid: Vec<(char, (u8, u8, u8))>) {
        if let Some(sender) = &mut self.sender {
            sender
                .send(RenderMessage::RenderWithColors { new_grid })
                .expect("Render channel is closed");
        }
    }

    /// Finish visualization and reset terminal
    pub fn finish(&mut self) {
        if self.finished {
            return;
        }

        // drop sender and wait for render thread to finish
        self.sender.take();
        if let Some(thread_handle) = self.thread_handle.take() {
            thread_handle.join().expect("Render thread panicked");
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

pub struct WindowedScreen {
    width: usize,
    height: usize,
    margin: Option<(usize, usize)>,
    last_window_top_left: Option<(usize, usize)>,
    screen: Screen,
    forward: bool,
}

impl WindowedScreen {
    /// Create a new windowed visualization with size `width * height`, the
    /// given frames per second, and an optional `margin` used to determine the
    /// position of the window around the center
    pub fn new(width: usize, height: usize, fps: u32, margin: Option<(usize, usize)>) -> Self {
        let (terminal_cols, terminal_rows) = terminal::size().unwrap();
        if terminal_cols as usize >= width && terminal_rows as usize >= height {
            // the terminal is large enough - we don't need a window
            let screen = Screen::new(terminal_cols as usize, terminal_rows as usize, fps);
            return Self {
                width,
                height,
                margin: None,
                last_window_top_left: None,
                screen,
                forward: true,
            };
        }

        if let Some(margin) = margin {
            if (terminal_rows as usize) < margin.1 * 2 + 1 {
                eprintln!(
                    "Terminal window is not high enough. Resize your terminal \
                    window (or zoom out), so at least {} rows can be displayed \
                    at once. Your terminal has a height of {terminal_rows} rows.",
                    margin.1 * 2 + 1
                );
                std::process::exit(1);
            }
            if (terminal_cols as usize) < margin.0 * 2 + 1 {
                eprintln!(
                    "Terminal window is not wide enough. Resize your terminal \
                    window (or zoom out), so at least {} columns can be displayed \
                    at once. Your terminal has a width of {terminal_cols} columns.",
                    margin.0 * 2 + 1
                );
                std::process::exit(1);
            }
        }

        let screen = Screen::new(terminal_cols as usize, terminal_rows as usize, fps);
        Self {
            width,
            height,
            margin,
            last_window_top_left: None,
            screen,
            forward: false,
        }
    }

    // Calculate the extent of the window
    fn get_window(&mut self, center: (usize, usize)) -> (usize, usize, usize, usize) {
        let center_min_x = center.0 as isize - self.screen.width as isize / 2;
        let center_max_x = center_min_x + self.screen.width as isize;
        let center_min_y = center.1 as isize - self.screen.height as isize / 2;
        let center_max_y = center_min_y + self.screen.height as isize;

        let (mut min_x, mut min_y, mut max_x, mut max_y) = if let Some(margin) = self.margin {
            if let Some(last_window_top_left) = self.last_window_top_left {
                let mut min_x = last_window_top_left.0 as isize;
                let mut min_y = last_window_top_left.1 as isize;
                if min_x + margin.0 as isize > center.0 as isize {
                    let dx = min_x + margin.0 as isize - center.0 as isize;
                    min_x -= dx;
                }
                if (center.0 as isize) > min_x + self.screen.width as isize - margin.0 as isize {
                    let dx = center.0 as isize
                        - (min_x + self.screen.width as isize - margin.0 as isize);
                    min_x += dx;
                }
                if min_y + margin.1 as isize > center.1 as isize {
                    let dy = min_y + margin.1 as isize - center.1 as isize;
                    min_y -= dy;
                }
                if (center.1 as isize) > min_y + self.screen.height as isize - margin.1 as isize {
                    let dy = center.1 as isize
                        - (min_y + self.screen.height as isize - margin.1 as isize);
                    min_y += dy;
                }
                (
                    min_x,
                    min_y,
                    min_x + self.screen.width as isize,
                    min_y + self.screen.height as isize,
                )
            } else {
                (center_min_x, center_min_y, center_max_x, center_max_y)
            }
        } else {
            (center_min_x, center_min_y, center_max_x, center_max_y)
        };

        if min_x < 0 {
            max_x -= min_x;
            min_x = 0;
        }
        if min_y < 0 {
            max_y -= min_y;
            min_y = 0;
        }
        if max_x > self.width as isize {
            min_x -= max_x - self.width as isize;
            max_x = self.width as isize;
        }
        if max_y > self.height as isize {
            min_y -= max_y - self.height as isize;
            max_y = self.height as isize;
        }

        self.last_window_top_left = Some((min_x as usize, min_y as usize));

        (
            min_x as usize,
            min_y as usize,
            max_x as usize,
            max_y as usize,
        )
    }

    /// Update the visualization with a new grid
    pub fn update(&mut self, new_grid: Vec<char>, center: (usize, usize)) {
        if self.forward {
            self.screen.update(new_grid);
        } else {
            let mut new_window = vec![' '; self.screen.width * self.screen.height];
            let (min_x, min_y, max_x, max_y) = self.get_window(center);
            for y in min_y..max_y {
                for x in min_x..max_x {
                    new_window[(y - min_y) * self.screen.width + (x - min_x)] =
                        new_grid[y * self.width + x];
                }
            }
            self.screen.update(new_window);
        }
    }

    /// Update the visualization with a new colored grid
    pub fn update_with_colors(
        &mut self,
        new_grid: Vec<(char, (u8, u8, u8))>,
        center: (usize, usize),
    ) {
        if self.forward {
            self.screen.update_with_colors(new_grid);
        } else {
            let mut new_window = vec![(' ', (0, 0, 0)); self.screen.width * self.screen.height];
            let (min_x, min_y, max_x, max_y) = self.get_window(center);
            for y in min_y..max_y {
                for x in min_x..max_x {
                    new_window[(y - min_y) * self.screen.width + (x - min_x)] =
                        new_grid[y * self.width + x];
                }
            }
            self.screen.update_with_colors(new_window);
        }
    }

    // Finish visualization and reset terminal
    pub fn finish(&mut self) {
        self.screen.finish();
    }
}
