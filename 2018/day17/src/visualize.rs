use screen::{
    WindowedScreen,
    style::{StyledContent, Stylize, style},
};

#[derive(Debug)]
struct StreamHead {
    x: usize,
    y: usize,
    volume: f32,
}

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

fn should_fill(
    x: usize,
    y: usize,
    grid: &[u8],
    new_grid: &[StyledContent<char>],
    width: usize,
    water: &StyledContent<char>,
) -> bool {
    grid[(y + 1) * width + x] == b'#' || new_grid[(y + 1) * width + x] == *water
}

pub fn visualize(grid: &[u8], width: usize, height: usize, start_x: usize) {
    let mut screen = WindowedScreen::new(width, height, 40, Some((20, 6)));

    let mut rng = Xorshift::new();
    let sand_colors = [(190, 142, 67), (228, 205, 145)];

    let mut new_grid = vec![style(' '); width * height];
    let mut wetness = vec![0.0; width * height];
    for y in 0..height {
        for x in 0..width {
            let c = match grid[y * width + x] {
                b'#' => style('█').with((130, 100, 45).into()),
                _ => {
                    let r = rng.next() % 800;
                    let col = if r > 50 {
                        (210, 177, 111)
                    } else {
                        sand_colors[(r as usize) % sand_colors.len()]
                    };
                    style('█').with(col.into()).on((210, 177, 111).into())
                }
            };
            new_grid[y * width + x] = c;
        }
    }

    let dried = style('█')
        .with((104, 104, 91).into())
        .on((128, 128, 75).into());
    let water_33 = style('█').with((197, 177, 134).into());
    let water_66 = style('█').with((163, 171, 180).into());
    let water = style('█').with((70, 127, 178).into());

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

            if should_fill(s.x, s.y, grid, &new_grid, width, &water) {
                let mut left = s.x;
                while left > 0
                    && grid[s.y * width + left - 1] != b'#'
                    && should_fill(left, s.y, grid, &new_grid, width, &water)
                {
                    left -= 1;
                }
                let mut right = s.x;
                while right < width - 1
                    && grid[s.y * width + right + 1] != b'#'
                    && should_fill(right, s.y, grid, &new_grid, width, &water)
                {
                    right += 1;
                }
                if !should_fill(left, s.y, grid, &new_grid, width, &water)
                    && !should_fill(right, s.y, grid, &new_grid, width, &water)
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
                } else if !should_fill(left, s.y, grid, &new_grid, width, &water) {
                    for x in left..=right {
                        new_grid[s.y * width + x] = dried;
                    }
                    s.x = left;
                    new_queue.push(s);
                } else if !should_fill(right, s.y, grid, &new_grid, width, &water) {
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
                    let fill = if max == 1.0 {
                        Some(water)
                    } else if max >= 0.66 {
                        Some(water_66)
                    } else if max >= 0.33 {
                        Some(water_33)
                    } else {
                        None
                    };
                    if let Some(fill) = fill {
                        for x in left..=right {
                            if fill == water || new_grid[s.y * width + x] != dried {
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
