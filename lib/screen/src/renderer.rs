use std::{
    io::{Stdout, stdout},
    thread,
    time::{Duration, Instant},
};

use crossterm::{
    ExecutableCommand, cursor,
    style::{self, Color, SetForegroundColor},
};

/// A message that can be sent to the render thread
pub(crate) enum RenderMessage {
    Render { new_grid: Vec<char> },
    RenderWithColors { new_grid: Vec<(char, (u8, u8, u8))> },
}

/// Renders grid to the screen
pub(crate) struct Renderer {
    width: usize,
    height: usize,
    time_per_frame: Duration,
    stdout: Stdout,
    pos: (u16, u16),
    last_grid: Vec<(char, Color)>,
    first_render: Option<Instant>,
    frames_rendered: u32,
}

impl Renderer {
    /// Create a new renderer
    pub fn new(width: usize, height: usize, time_per_frame: Duration, pos: (u16, u16)) -> Self {
        let last_grid = vec![(' ', Color::Grey); width * height];
        Self {
            width,
            height,
            time_per_frame,
            stdout: stdout(),
            pos,
            last_grid,
            first_render: None,
            frames_rendered: 0,
        }
    }

    /// Try to sleep between frames to maintain fps. Return `false` if rendering
    /// should be skipped.
    fn try_sleep(&mut self, render_queue_len: usize) -> bool {
        let mut result = true;

        if let Some(first_render) = self.first_render {
            let elapsed = first_render.elapsed();
            if self.time_per_frame * self.frames_rendered > elapsed {
                thread::sleep(self.time_per_frame * self.frames_rendered - elapsed);
            } else {
                let delay = elapsed - self.time_per_frame * self.frames_rendered;
                if delay >= self.time_per_frame && render_queue_len > 0 {
                    // We're way too late and there are other frames waiting.
                    // Better skip this frame to keep up.
                    result = false;
                }
            }
        } else {
            self.first_render = Some(Instant::now());
        }
        self.frames_rendered += 1;

        result
    }

    /// Render a grid to the screen
    pub fn render(&mut self, msg: RenderMessage, render_queue_len: usize) {
        if !self.try_sleep(render_queue_len) {
            return;
        }

        let mut last_cursor_x = usize::MAX;
        let mut last_cursor_y = usize::MAX;
        let mut stdout = self.stdout.lock();

        match msg {
            RenderMessage::Render { new_grid } => {
                for y in 0..self.height {
                    for x in 0..self.width {
                        let c = new_grid[y * self.width + x];
                        if c != self.last_grid[y * self.width + x].0 {
                            self.last_grid[y * self.width + x].0 = c;
                            if last_cursor_x != x || last_cursor_y != y {
                                stdout
                                    .execute(cursor::MoveTo(
                                        self.pos.0 + x as u16,
                                        self.pos.1 + y as u16,
                                    ))
                                    .unwrap();
                            }
                            stdout.execute(style::Print(c)).unwrap();
                            last_cursor_y = y;
                            last_cursor_x = x + 1;
                        }
                    }
                }
            }

            RenderMessage::RenderWithColors { new_grid } => {
                let mut last_color = Color::Grey;

                for y in 0..self.height {
                    for x in 0..self.width {
                        let (r, g, b) = new_grid[y * self.width + x].1;
                        let c = (new_grid[y * self.width + x].0, Color::Rgb { r, g, b });
                        if c != self.last_grid[y * self.width + x] {
                            self.last_grid[y * self.width + x] = c;
                            if c.1 != last_color {
                                stdout.execute(SetForegroundColor(c.1)).unwrap();
                                last_color = c.1;
                            }
                            if last_cursor_x != x || last_cursor_y != y {
                                stdout
                                    .execute(cursor::MoveTo(
                                        self.pos.0 + x as u16,
                                        self.pos.1 + y as u16,
                                    ))
                                    .unwrap();
                            }
                            stdout.execute(style::Print(c.0)).unwrap();
                            last_cursor_y = y;
                            last_cursor_x = x + 1;
                        }
                    }
                }

                stdout.execute(SetForegroundColor(Color::Grey)).unwrap();
            }
        }
    }
}
