use std::{
    io::{Stdout, stdout},
    thread,
    time::{Duration, Instant},
};

use crossterm::{
    ExecutableCommand, cursor,
    style::{
        self, Attributes, Color, ContentStyle, SetAttributes, SetBackgroundColor,
        SetForegroundColor, SetUnderlineColor, StyledContent,
    },
};

/// A message that can be sent to the render thread
pub(crate) enum RenderMessage {
    Render { new_grid: Vec<char> },
    RenderWithColors { new_grid: Vec<(char, (u8, u8, u8))> },
    RenderWithStyle { new_grid: Vec<StyledContent<char>> },
}

/// Renders grid to the screen
pub(crate) struct Renderer {
    width: usize,
    height: usize,
    time_per_frame: Duration,
    stdout: Stdout,
    pos: (u16, u16),
    last_grid: Vec<(char, ContentStyle)>,
    first_render: Option<Instant>,
    frames_rendered: u32,
}

impl Renderer {
    /// Create a new renderer
    pub fn new(width: usize, height: usize, time_per_frame: Duration, pos: (u16, u16)) -> Self {
        let last_grid = vec![(' ', ContentStyle::new()); width * height];
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
                        let fg = Color::Rgb { r, g, b };
                        let mut cs = ContentStyle::new();
                        cs.foreground_color = Some(fg);
                        let c = (new_grid[y * self.width + x].0, cs);
                        if c != self.last_grid[y * self.width + x] {
                            self.last_grid[y * self.width + x] = c;
                            if fg != last_color {
                                stdout.execute(SetForegroundColor(fg)).unwrap();
                                last_color = fg;
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

            RenderMessage::RenderWithStyle { new_grid } => {
                let mut last_color = Color::Grey;
                let mut last_background = Color::Reset;
                let mut last_underline = Color::Reset;
                let mut last_attributes = Attributes::none();

                for y in 0..self.height {
                    for x in 0..self.width {
                        let c = new_grid[y * self.width + x];
                        let c = (*c.content(), *c.style());
                        if c != self.last_grid[y * self.width + x] {
                            self.last_grid[y * self.width + x] = c;
                            if let Some(fg) = c.1.foreground_color
                                && fg != last_color
                            {
                                stdout.execute(SetForegroundColor(fg)).unwrap();
                                last_color = fg;
                            }
                            if let Some(bg) = c.1.background_color
                                && bg != last_background
                            {
                                stdout.execute(SetBackgroundColor(bg)).unwrap();
                                last_background = bg;
                            }
                            if let Some(ul) = c.1.underline_color
                                && ul != last_underline
                            {
                                stdout.execute(SetUnderlineColor(ul)).unwrap();
                                last_underline = ul;
                            }
                            if c.1.attributes != last_attributes {
                                stdout.execute(SetAttributes(c.1.attributes)).unwrap();
                                last_attributes = c.1.attributes;
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
                stdout.execute(SetBackgroundColor(Color::Reset)).unwrap();
                stdout.execute(SetAttributes(Attributes::none())).unwrap();
            }
        }
    }
}
