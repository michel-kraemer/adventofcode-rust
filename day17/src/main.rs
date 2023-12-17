use std::{
    cmp::{Ordering, Reverse},
    collections::{BinaryHeap, HashSet},
    fs,
};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
struct Node {
    x: i32,
    y: i32,
    dir_x: i32,
    dir_y: i32,
    steps: usize,
    cost: usize,
}

impl Ord for Node {
    fn cmp(&self, other: &Self) -> Ordering {
        self.cost.cmp(&other.cost)
    }
}

impl PartialOrd for Node {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(&other))
    }
}

fn main() {
    for part1 in [true, false] {
        let min_steps = if part1 { 0 } else { 4 };
        let max_steps = if part1 { 3 } else { 10 };

        let input = fs::read_to_string("input.txt").expect("Could not read file");
        let grid: Vec<Vec<usize>> = input
            .lines()
            .map(|line| {
                line.chars()
                    .map(|c| c.to_digit(10).unwrap() as usize)
                    .collect()
            })
            .collect();

        let mut queue = BinaryHeap::new();
        let mut seen = HashSet::new();

        queue.push(Reverse(Node {
            x: 0,
            y: 0,
            dir_x: 0,
            dir_y: 0,
            steps: 0,
            cost: 0,
        }));

        while !queue.is_empty() {
            let current = queue.pop().unwrap().0;

            let sc = Node { cost: 0, ..current };
            if seen.contains(&sc) {
                continue;
            }
            seen.insert(sc);

            if current.x as usize == grid[0].len() - 1 && current.y as usize == grid.len() - 1 {
                println!("{}", current.cost);
                break;
            }

            let mut successors = Vec::new();
            if current.steps < max_steps && !(current.dir_x == 0 && current.dir_y == 0) {
                successors.push(Node {
                    x: current.x + current.dir_x,
                    y: current.y + current.dir_y,
                    dir_x: current.dir_x,
                    dir_y: current.dir_y,
                    steps: current.steps + 1,
                    cost: 0,
                });
            }

            if current.steps >= min_steps || (current.dir_x == 0 && current.dir_y == 0) {
                if current.dir_x == 0 {
                    successors.push(Node {
                        x: current.x + 1,
                        y: current.y,
                        dir_x: 1,
                        dir_y: 0,
                        steps: 1,
                        cost: 0,
                    });
                    successors.push(Node {
                        x: current.x - 1,
                        y: current.y,
                        dir_x: -1,
                        dir_y: 0,
                        steps: 1,
                        cost: 0,
                    });
                }
                if current.dir_y == 0 {
                    successors.push(Node {
                        x: current.x,
                        y: current.y + 1,
                        dir_x: 0,
                        dir_y: 1,
                        steps: 1,
                        cost: 0,
                    });
                    successors.push(Node {
                        x: current.x,
                        y: current.y - 1,
                        dir_x: 0,
                        dir_y: -1,
                        steps: 1,
                        cost: 0,
                    });
                }
            }

            for mut successor in successors {
                if successor.x < 0
                    || successor.y < 0
                    || successor.x >= grid[0].len() as i32
                    || successor.y >= grid.len() as i32
                {
                    continue;
                }
                successor.cost = current.cost + grid[successor.y as usize][successor.x as usize];
                queue.push(Reverse(successor));
            }
        }
    }
}
