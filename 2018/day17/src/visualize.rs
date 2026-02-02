use screen::{
    WindowedScreen,
    style::{Stylize, style},
};

#[derive(Debug)]
struct StreamHead {
    x: usize,
    y: usize,
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

fn should_fill(x: usize, y: usize, grid: &[u8], wetness: &[f32], width: usize) -> bool {
    grid[(y + 1) * width + x] == b'#' || wetness[(y + 1) * width + x] == 1.0
}

pub fn visualize(grid: &[u8], width: usize, height: usize, start_x: usize) {
    let dried = style('█').with((104, 104, 91).into());
    let water_33 = style('█').with((197, 177, 134).into());
    let water_66 = style('█').with((163, 171, 180).into());
    let water = style('█').with((95, 158, 211).into());
    let water_darker_33 = style('█').with((171, 144, 97).into());
    let water_darker_66 = style('█').with((129, 139, 150).into());
    let water_darker = style('█').with((70, 127, 178).into());
    let water_lighter_33 = style('█').with((219, 205, 164).into());
    let water_lighter_66 = style('█').with((196, 200, 200).into());
    let water_lighter = style('█').with((142, 195, 233).into());
    let sand = style('█').with((210, 177, 111).into());
    let sand_darker = style('█').with((190, 142, 67).into());
    let sand_lighter = style('█').with((228, 205, 145).into());

    let mut screen = WindowedScreen::new(width, height, 40, Some((20, 6)));
    let mut rng = Xorshift::new();

    let mut new_grid = vec![style(' '); width * height];
    let mut wetness = vec![0.0; width * height];

    for y in 0..height {
        for x in 0..width {
            let c = match grid[y * width + x] {
                b'#' => style('█').with((130, 100, 45).into()),
                _ => {
                    let r = rng.next() % 800;
                    if r > 50 {
                        sand
                    } else if r.is_multiple_of(2) {
                        sand_darker
                    } else {
                        sand_lighter
                    }
                }
            };
            new_grid[y * width + x] = c;
        }
    }

    let unmodified_new_grid = new_grid.clone();

    let mut last_max_x = 0;
    let mut last_max_y = 0;

    let mut queue = vec![StreamHead {
        x: start_x,
        y: 0,
        volume: 1.0,
    }];
    for _ in 0.. {
        let mut new_queue = Vec::new();
        for mut s in queue {
            if s.y == height - 1 {
                new_grid[s.y * width + s.x] = dried;
                continue;
            }

            if should_fill(s.x, s.y, grid, &wetness, width) {
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
                    for x in left..=right {
                        new_grid[s.y * width + x] = dried;
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
                    for x in left..=right {
                        new_grid[s.y * width + x] = dried;
                    }
                    s.x = left;
                    new_queue.push(s);
                } else if !should_fill(right, s.y, grid, &wetness, width) {
                    for x in left..=right {
                        new_grid[s.y * width + x] = dried;
                    }
                    s.x = right;
                    new_queue.push(s);
                } else {
                    let mut max = 0.0_f32;
                    for x in left..=right {
                        let w = 1.0_f32.min(wetness[s.y * width + x] + s.volume);
                        wetness[s.y * width + x] = w;
                        max = max.max(w);
                    }
                    if max >= 0.33 {
                        for x in left..=right {
                            if max == 1.0 || new_grid[s.y * width + x] != dried {
                                let ug = unmodified_new_grid[s.y * width + x];
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
                                new_grid[s.y * width + x] = fill;
                            }
                        }
                    } else if grid[(s.y + 1) * width + s.x] == b'#' {
                        new_grid[s.y * width + s.x] = dried;
                    }
                    new_queue.push(s);
                }
            } else {
                new_grid[s.y * width + s.x] = dried;
                s.y += 1;
                new_queue.push(s);
            }
        }
        queue = new_queue;

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

        queue.sort_by_key(|s| (s.y, s.x));
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

        if queue.is_empty() {
            break;
        }

        let max_s = queue.last().unwrap();
        screen.update_with_style(new_grid.clone(), (max_s.x, max_s.y));
        last_max_x = max_s.x;
        last_max_y = max_s.y;
    }

    screen.update_with_style(new_grid.clone(), (last_max_x, last_max_y));

    drop(screen);

    println!();
}
