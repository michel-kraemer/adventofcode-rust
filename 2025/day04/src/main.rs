use std::{collections::VecDeque, env, fs};

use crate::screen::Screen;

mod screen;

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

#[derive(Default)]
struct Stack(Vec<(usize, usize)>);

#[derive(Default)]
struct Queue(VecDeque<(usize, usize)>);

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

fn run<T: StackOrQueue>(visualize: bool, mut queue: T) {
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

    let mut screen = if visualize {
        Some(Screen::new(width, height))
    } else {
        None
    };

    // Part 2: For each of the rolls in the queue, remove them and decrease the
    // count of all their neighbors. If the count of a neighbor falls below the
    // limit, add it to the queue too.
    let mut total2 = 0;
    while let Some((x, y)) = queue.pop() {
        if let Some(screen) = &mut screen {
            counts[y * width + x] = 0;
            screen.update(&counts);
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

    if let Some(mut screen) = screen {
        screen.finish();
    }

    println!("{total2}");
}

fn main() {
    let visualize = env::var("AOC_VISUALIZE").is_ok();
    if visualize {
        run(true, Stack::default());
        run(true, Queue::default());
    } else {
        run(false, Queue::default());
    }
}
