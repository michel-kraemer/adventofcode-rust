use std::{collections::VecDeque, fs};

#[cfg(feature = "visualize")]
use screen::Screen;

pub const CLOCKWISE: [(i64, i64); 8] = [
    (1, 0),   // →
    (1, 1),   // ↘︎
    (0, 1),   // ↓
    (-1, 1),  // ↙︎
    (-1, 0),  // ←
    (-1, -1), // ↖︎
    (0, -1),  // ↑
    (1, -1),  // ↗︎
];

trait StackOrQueue {
    fn push(&mut self, xy: (usize, usize));
    fn pop(&mut self) -> Option<(usize, usize)>;
}

#[cfg(feature = "visualize")]
#[derive(Default)]
struct Stack(Vec<(usize, usize)>);

#[derive(Default)]
struct Queue(VecDeque<(usize, usize)>);

#[cfg(feature = "visualize")]
impl StackOrQueue for Stack {
    fn push(&mut self, xy: (usize, usize)) {
        self.0.push(xy);
    }

    fn pop(&mut self) -> Option<(usize, usize)> {
        self.0.pop()
    }
}

impl StackOrQueue for Queue {
    fn push(&mut self, xy: (usize, usize)) {
        self.0.push_back(xy);
    }

    fn pop(&mut self) -> Option<(usize, usize)> {
        self.0.pop_front()
    }
}

fn run<T: StackOrQueue>(mut queue: T) {
    let input = fs::read_to_string("input.txt").expect("Could not read file");
    let lines = input.lines().collect::<Vec<_>>();
    let width = lines[0].len();
    let height = lines.len();
    let grid = lines
        .iter()
        .flat_map(|l| l.as_bytes())
        .copied()
        .collect::<Vec<_>>();

    // Part 1: Look for rolls of paper we can access. Put them into a queue for
    // part 2.
    let mut counts = vec![0u8; grid.len()];
    let mut total1 = 0;
    for y in 0..height {
        for x in 0..width {
            if grid[y * width + x] == b'@' {
                // optimization: just look in four directions (→↘︎↓↙︎) and
                // update the counts of our neighbors too
                for (dx, dy) in &CLOCKWISE[0..4] {
                    let nx = x as i64 + dx;
                    let ny = y as i64 + dy;
                    if nx >= 0
                        && (nx as usize) < width
                        && ny >= 0
                        && (ny as usize) < height
                        && grid[ny as usize * width + nx as usize] == b'@'
                    {
                        counts[y * width + x] += 1;
                        counts[ny as usize * width + nx as usize] += 1;
                    }
                }
                if counts[y * width + x] < 4 {
                    total1 += 1;
                    queue.push((x, y));
                }
            }
        }
    }
    println!("{total1}");

    #[cfg(feature = "visualize")]
    let mut screen = Screen::new(width, height.div_ceil(2), 800);

    // Part 2: For each of the rolls in the queue, remove them and decrease the
    // count of all their neighbors. If the count of a neighbor falls below the
    // limit, add it to the queue too.
    let mut total2 = 0;
    while let Some((x, y)) = queue.pop() {
        #[cfg(feature = "visualize")]
        {
            counts[y * width + x] = 0;
            visualize(&counts, &mut screen, height);
        }

        total2 += 1;
        for (dx, dy) in CLOCKWISE {
            let nx = x as i64 + dx;
            let ny = y as i64 + dy;
            if nx >= 0
                && (nx as usize) < width
                && ny >= 0
                && (ny as usize) < height
                && counts[ny as usize * width + nx as usize] >= 4
            {
                if counts[ny as usize * width + nx as usize] == 4 {
                    // Add neighbor to queue. Since we check for == 4, instead
                    // of <4, this will only happen once and the queue will
                    // contain no duplicates.
                    queue.push((nx as usize, ny as usize));
                }
                counts[ny as usize * width + nx as usize] -= 1;
            }
        }
    }

    #[cfg(feature = "visualize")]
    drop(screen);

    println!("{total2}");
}

#[cfg(feature = "visualize")]
fn visualize(counts: &[u8], screen: &mut Screen, height: usize) {
    let mut new_grid = vec![' '; screen.width() * screen.height()];
    for y in (0..height).step_by(2) {
        for x in 0..screen.width() {
            let t = counts[y * screen.width() + x] > 0;
            let b = if y < height - 1 {
                counts[(y + 1) * screen.width() + x] > 0
            } else {
                false
            };
            let c = match (t, b) {
                (true, true) => '█',
                (true, false) => '▀',
                (false, true) => '▄',
                (false, false) => ' ',
            };
            new_grid[y / 2 * screen.width() + x] = c;
        }
    }
    screen.update(new_grid);
}

fn main() {
    #[cfg(feature = "visualize")]
    run(Stack::default());
    run(Queue::default());
}
