use std::{
    collections::{BinaryHeap, HashSet},
    fs,
};

#[derive(Eq, PartialEq, Hash)]
struct Node {
    heat_loss: u32,
    x: i32,
    y: i32,
    steps: u32,
    dx: i32,
    dy: i32,
}

impl PartialOrd for Node {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for Node {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        other.heat_loss.cmp(&self.heat_loss)
    }
}

fn main() {
    for part1 in [true, false] {
        let min_steps = if part1 { 1 } else { 4 };
        let max_steps = if part1 { 3 } else { 10 };

        let input = fs::read_to_string("input.txt").expect("Could not read file");
        let grid = input
            .lines()
            .map(|l| {
                l.chars()
                    .map(|c| c.to_digit(10).unwrap())
                    .collect::<Vec<_>>()
            })
            .collect::<Vec<_>>();

        let mut queue = BinaryHeap::new();
        queue.push(Node {
            heat_loss: 0,
            x: 0,
            y: 0,
            steps: 0,
            dx: 0,
            dy: 0,
        });
        let mut seen = HashSet::new();

        while let Some(p) = queue.pop() {
            if p.x == grid[0].len() as i32 - 1
                && p.y == grid.len() as i32 - 1
                && p.steps >= min_steps
            {
                println!("{}", p.heat_loss);
                break;
            }

            for (dx, dy) in [(0, 1), (0, -1), (1, 0), (-1, 0)] {
                if dx == -p.dx && dy == -p.dy {
                    continue;
                }

                let x = p.x + dx;
                let y = p.y + dy;
                if x < 0 || y < 0 || x >= grid[0].len() as i32 || y >= grid.len() as i32 {
                    continue;
                }

                let heat_loss = p.heat_loss + grid[y as usize][x as usize];

                let steps = if p.steps == 0 || (dx == p.dx && dy == p.dy) {
                    if p.steps == max_steps {
                        // don't go further ahead if we've already taken max_steps
                        continue;
                    }
                    p.steps + 1
                } else {
                    if p.steps < min_steps {
                        // don't turn if we haven't taken min_steps yet
                        continue;
                    }
                    1
                };

                let nk = Node {
                    heat_loss: 0,
                    x,
                    y,
                    steps,
                    dx,
                    dy,
                };
                if !seen.contains(&nk) {
                    queue.push(Node { heat_loss, ..nk });
                    seen.insert(nk);
                }
            }
        }
    }
}
