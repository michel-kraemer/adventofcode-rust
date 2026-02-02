use screen::{
    WindowedScreen,
    style::{Color, StyledContent, Stylize, style},
};

/// The head of a stream of water
#[derive(Debug)]
struct StreamHead {
    /// The stream head's position
    x: usize,

    /// The stream head's position
    y: usize,

    /// The stream's water volume (used to calculate how quickly rows fill up)
    volume: f32,
}

/// Simple pseudo-random generator. See https://en.wikipedia.org/wiki/Xorshift.
struct Xorshift {
    x32: u32,
}

impl Xorshift {
    fn new() -> Self {
        Self { x32: 314159265 }
    }

    fn next(&mut self) -> u32 {
        self.x32 ^= self.x32 << 13;
        self.x32 ^= self.x32 >> 17;
        self.x32 ^= self.x32 << 5;
        self.x32
    }
}

/// A grid that encodes two vertical blocks in one block using different
/// foreground and background colors
#[derive(Clone)]
struct HalfGrid {
    width: usize,
    grid: Vec<StyledContent<char>>,
}

impl HalfGrid {
    fn new(width: usize, height: usize) -> Self {
        Self {
            width,
            grid: vec![style('▀').with(Color::Grey).on(Color::Reset); width * height.div_ceil(2)],
        }
    }

    fn set(&mut self, x: usize, y: usize, col: Color) {
        let yh = y / 2;
        let s = self.grid[yh * self.width + x].style_mut();
        if y.is_multiple_of(2) {
            s.foreground_color = Some(col);
        } else {
            s.background_color = Some(col);
        }
    }

    fn get(&self, x: usize, y: usize) -> Color {
        let yh = y / 2;
        let s = self.grid[yh * self.width + x].style();
        if y.is_multiple_of(2) {
            s.foreground_color.unwrap_or(Color::Reset)
        } else {
            s.background_color.unwrap_or(Color::Reset)
        }
    }
}

/// True if a stream head is in a row that it should fill with water
fn should_fill(x: usize, y: usize, grid: &[u8], wetness: &[f32], width: usize) -> bool {
    grid[(y + 1) * width + x] == b'#' || wetness[(y + 1) * width + x] == 1.0
}

/// Visualize the puzzle
pub fn visualize(grid: &[u8], width: usize, height: usize, start_x: usize) {
    // grid colors
    let dried = Color::from((104, 104, 91));
    let water_33 = Color::from((197, 177, 134));
    let water_66 = Color::from((163, 171, 180));
    let water = Color::from((95, 158, 211));
    let water_darker_33 = Color::from((171, 144, 97));
    let water_darker_66 = Color::from((129, 139, 150));
    let water_darker = Color::from((70, 127, 178));
    let water_lighter_33 = Color::from((219, 205, 164));
    let water_lighter_66 = Color::from((196, 200, 200));
    let water_lighter = Color::from((142, 195, 233));
    let sand = Color::from((210, 177, 111));
    let sand_darker = Color::from((190, 142, 67));
    let sand_lighter = Color::from((228, 205, 145));

    // create windowed screen
    let fps = 40;
    let mut screen = WindowedScreen::new(width, height.div_ceil(2), fps, Some((20, 6)));

    // create pseudo-random number generator
    let mut rng = Xorshift::new();

    // create grid for the screen and a vec storing how "wet" each row is (1.0
    // means 100% wet)
    let mut new_grid = HalfGrid::new(width, height);
    let mut wetness = vec![0.0; width * height];

    // convert original grid to coloured grid
    for y in 0..height {
        for x in 0..width {
            let c = match grid[y * width + x] {
                b'#' => Color::from((130, 100, 45)),
                _ => {
                    let r = rng.next() % 800; // normal sand is much more likely
                    if r > 50 {
                        sand
                    } else if r.is_multiple_of(2) {
                        sand_darker
                    } else {
                        sand_lighter
                    }
                }
            };
            new_grid.set(x, y, c);
        }
    }

    // save unmodified colored grid for faster comparisons
    let unmodified_new_grid = new_grid.clone();

    let mut camera_x = 0;
    let mut last_max_x = 0;
    let mut last_max_y = 0;

    // start with one stream head
    let mut queue = vec![StreamHead {
        x: start_x,
        y: 0,
        volume: 1.0,
    }];
    loop {
        // Iterate each stream head, fill rows, and create new stream heads if
        // necessary. Works similar to the puzzle solution but in a BFS manner.
        let mut new_queue = Vec::new();
        for mut s in queue {
            if s.y == height - 1 {
                // stream head has reach the end of the grid
                new_grid.set(s.x, s.y, dried);
                continue;
            }

            if should_fill(s.x, s.y, grid, &wetness, width) {
                // Stream head is in a row that should be filled. Look for left
                // and right borders or places where the water drops down.
                let mut left = s.x;
                while left > 0
                    && grid[s.y * width + left - 1] != b'#'
                    && should_fill(left, s.y, grid, &wetness, width)
                {
                    left -= 1;
                }
                let mut right = s.x;
                while right < width - 1
                    && grid[s.y * width + right + 1] != b'#'
                    && should_fill(right, s.y, grid, &wetness, width)
                {
                    right += 1;
                }

                if !should_fill(left, s.y, grid, &wetness, width)
                    && !should_fill(right, s.y, grid, &wetness, width)
                {
                    // Water drops down at both sides. Colorize the current row
                    // with "dried" color and create two new stream heads with
                    // half the volume each.
                    for x in left..=right {
                        new_grid.set(x, s.y, dried);
                    }
                    new_queue.push(StreamHead {
                        x: left,
                        y: s.y,
                        volume: s.volume / 2.0,
                    });
                    new_queue.push(StreamHead {
                        x: right,
                        y: s.y,
                        volume: s.volume / 2.0,
                    });
                } else if !should_fill(left, s.y, grid, &wetness, width) {
                    // Water drops down on the left. Colorize current row and
                    // move stream head.
                    for x in left..=right {
                        new_grid.set(x, s.y, dried);
                    }
                    s.x = left;
                    new_queue.push(s);
                } else if !should_fill(right, s.y, grid, &wetness, width) {
                    // Water drops down on the right. Colorize current row and
                    // move stream head.
                    for x in left..=right {
                        new_grid.set(x, s.y, dried);
                    }
                    s.x = right;
                    new_queue.push(s);
                } else {
                    // We found borders at the left and at the right. Fill the
                    // current row with water
                    let mut max = 0.0_f32;
                    for x in left..=right {
                        let w = 1.0_f32.min(wetness[s.y * width + x] + s.volume);
                        wetness[s.y * width + x] = w;
                        max = max.max(w);
                    }

                    if max >= 0.33 {
                        // Colorize the current row depending on its wetness.
                        // Take existing sand colour into account so that we get
                        // a nice blending effect.
                        for x in left..=right {
                            if max == 1.0 || new_grid.get(x, s.y) != dried {
                                let ug = unmodified_new_grid.get(x, s.y);
                                let fill = if ug == sand_darker {
                                    if max == 1.0 {
                                        water_darker
                                    } else if max >= 0.66 {
                                        water_darker_66
                                    } else {
                                        water_darker_33
                                    }
                                } else if ug == sand_lighter {
                                    if max == 1.0 {
                                        water_lighter
                                    } else if max >= 0.66 {
                                        water_lighter_66
                                    } else {
                                        water_lighter_33
                                    }
                                } else if max == 1.0 {
                                    water
                                } else if max >= 0.66 {
                                    water_66
                                } else {
                                    water_33
                                };
                                new_grid.set(x, s.y, fill);
                            }
                        }
                    } else if grid[(s.y + 1) * width + s.x] == b'#' {
                        // draw current stream head
                        new_grid.set(s.x, s.y, dried);
                    }
                    new_queue.push(s);
                }
            } else {
                // Water is still falling. Move stream head down.
                new_grid.set(s.x, s.y, dried);
                s.y += 1;
                new_queue.push(s);
            }
        }
        queue = new_queue;

        if queue.is_empty() {
            // nothing more to visualize
            break;
        }

        // move stream head up if the row it is filling is already fully wet
        for s in &mut queue {
            if wetness[s.y * width + s.x] > 0.0 {
                let mut left = s.x;
                while left > 0 && grid[s.y * width + left - 1] != b'#' {
                    left -= 1;
                }
                let mut right = s.x;
                while right < width - 1 && grid[s.y * width + right + 1] != b'#' {
                    right += 1;
                }
                let all_wet = (left..=right).all(|x| wetness[s.y * width + x] == 1.0);
                if all_wet {
                    s.y -= 1;
                }
            }
        }

        // sort points by y position so we can select the lowest point for our
        // camera position later.
        queue.sort_by_key(|s| (s.y, -(s.x as isize)));

        // sorted points can also be used to merge duplicates
        let mut i = 0;
        while i < queue.len() {
            while i < queue.len() - 1
                && queue[i + 1].x == queue[i].x
                && queue[i + 1].y == queue[i].y
            {
                let v = queue[i].volume;
                queue.remove(i);
                queue[i].volume += v;
            }
            i += 1;
        }

        // ease camera in x direction
        let max_s = queue.last().unwrap();
        if camera_x == 0 {
            camera_x = max_s.x;
        } else {
            let v = fps as f32 / (1000.0 / 500.0);
            if max_s.x != camera_x {
                let dx = max_s.x as f32 - camera_x as f32;
                let mut inc = dx / v;
                if inc > 0.0 && inc < 1.0 {
                    inc = 1.0;
                }
                if inc < 0.0 && inc > -1.0 {
                    inc = -1.0;
                }
                camera_x = camera_x.saturating_add_signed(inc as isize);
            }
        }

        // update screen
        screen.update_with_style(new_grid.grid.clone(), (camera_x, max_s.y / 2));
        last_max_x = max_s.x;
        last_max_y = max_s.y;
    }

    // perform final update
    screen.update_with_style(new_grid.grid.clone(), (last_max_x, last_max_y));

    drop(screen);

    println!();
}
