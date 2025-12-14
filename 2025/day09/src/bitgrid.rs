pub struct BitGrid {
    words: usize,
    grid: Vec<u64>,
}

impl BitGrid {
    pub fn new(width: usize, height: usize) -> Self {
        let words = width.div_ceil(64);
        Self {
            words,
            grid: vec![0; words * height],
        }
    }

    pub fn set(&mut self, x: usize, y: usize) {
        let w = x / 64;
        let i = x % 64;
        self.grid[y * self.words + w] |= 1 << i;
    }

    pub fn get(&self, x: usize, y: usize) -> bool {
        let w = x / 64;
        let i = x % 64;
        (self.grid[y * self.words + w] & (1 << i)) > 0
    }

    pub fn fill(&mut self, y: usize, minx: usize, maxx: usize) {
        let minw = minx / 64;
        let maxw = maxx / 64;
        let mini = (minx % 64) as u64;
        let maxi = (maxx % 64) as u64;
        if minw == maxw {
            self.grid[y * self.words + minw] |= ((1 << (maxi + 1)) - 1) & !((1 << mini) - 1);
        } else {
            self.grid[y * self.words + minw] |= !((1 << mini) - 1);
            if maxw - minw > 1 {
                self.grid[y * self.words + minw + 1..=y * self.words + maxw - 1].fill(u64::MAX);
            }
            self.grid[y * self.words + maxw] |= (1 << (maxi + 1)) - 1;
        }
    }

    pub fn is_rect_filled(&self, minx: usize, miny: usize, maxx: usize, maxy: usize) -> bool {
        let minw = minx / 64;
        let maxw = maxx / 64;
        let mini = (minx % 64) as u64;
        let maxi = (maxx % 64) as u64;
        if minw == maxw {
            let mask = ((1 << (maxi + 1)) - 1) & !((1 << mini) - 1);
            for y in miny..=maxy {
                if self.grid[y * self.words + minw] & mask != mask {
                    return false;
                }
            }
        } else {
            // performance optimization: check vertical edges first
            let min_mask = !((1 << mini) - 1);
            let max_mask = (1 << (maxi + 1)) - 1;
            for y in miny..=maxy {
                if self.grid[y * self.words + minw] & min_mask != min_mask {
                    return false;
                }
                if self.grid[y * self.words + maxw] & max_mask != max_mask {
                    return false;
                }
            }

            if maxw - minw > 1 {
                // now check the inner of the rectangle
                for y in miny..=maxy {
                    if self
                        .grid
                        .iter()
                        .skip(y * self.words + minw + 1)
                        .take(maxw - minw - 1)
                        .any(|b| *b != u64::MAX)
                    {
                        return false;
                    }
                }
            }
        }

        true
    }
}
