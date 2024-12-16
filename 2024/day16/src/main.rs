use std::cmp::Reverse;
use std::collections::{BinaryHeap, VecDeque};
use std::fs;

const DIRS: [(i32, i32); 4] = [(1, 0), (0, 1), (-1, 0), (0, -1)];

#[derive(PartialEq, Eq)]
struct State {
    score: usize,
    x: i32,
    y: i32,
    di: usize,
}

impl Ord for State {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.score.cmp(&other.score)
    }
}

impl PartialOrd for State {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

fn main() {
    let input = fs::read_to_string("input.txt").expect("Could not read file");
    let lines = input.lines().collect::<Vec<_>>();
    let width = lines[0].len();
    let height = lines.len();
    let grid = lines
        .iter()
        .flat_map(|l| l.as_bytes())
        .copied()
        .collect::<Vec<_>>();

    // find start and end
    let mut start = (0, 0);
    let mut end = (0, 0);
    for y in 0..height {
        for x in 0..width {
            if grid[y * width + x] == b'S' {
                start = (x as i32, y as i32);
            } else if grid[y * width + x] == b'E' {
                end = (x as i32, y as i32);
            }
        }
    }

    let mut heap = BinaryHeap::new();
    let start_state = State {
        score: 0,
        x: start.0,
        y: start.1,
        di: 0, // start facing east
    };
    heap.push(Reverse(start_state));

    // a mapping of seen places and directions to their lowest score
    let mut seen = vec![usize::MAX; width * height * DIRS.len()];

    // the minimum score it takes to get to the end
    let mut min = usize::MAX;

    while let Some(Reverse(State {
        score,
        x,
        y,
        di: prev_di,
    })) = heap.pop()
    {
        if grid[y as usize * width + x as usize] == b'E' {
            if score > min {
                // This path is worse than any best path we've found before.
                // We can break here. No other path will be better.
                break;
            }
            min = score;
        }

        for (di, (dx, dy)) in DIRS.iter().enumerate() {
            if (prev_di + 2) % DIRS.len() == di {
                // don't go back
                continue;
            }

            let nscore = if di == prev_di {
                // walk forwards
                score + 1
            } else {
                // turn and take one step
                score + 1001
            };

            let nx = x + dx;
            let ny = y + dy;

            let gi = ny as usize * width + nx as usize;
            let si = gi * DIRS.len() + di;
            let last_seen_score = seen[si];

            if grid[gi] != b'#' && nscore <= last_seen_score {
                // save score for this place and direction
                seen[si] = nscore;

                heap.push(Reverse(State {
                    score: nscore,
                    x: nx,
                    y: ny,
                    di,
                }));
            }
        }
    }

    // backtrack best paths from the end position to the start
    let mut total = 1; // include end position
    let mut places_to_sit = vec![false; width * height];
    let mut queue = VecDeque::new();
    queue.push_back((end, min));
    while let Some((node, score)) = queue.pop_front() {
        for di in 0..DIRS.len() {
            let next_score = seen[(node.1 as usize * width + node.0 as usize) * DIRS.len() + di];
            if next_score <= score {
                // walk back
                let nextx = node.0 - DIRS[di].0;
                let nexty = node.1 - DIRS[di].1;

                if !places_to_sit[nexty as usize * width + nextx as usize] {
                    places_to_sit[nexty as usize * width + nextx as usize] = true;
                    total += 1;
                    queue.push_back(((nextx, nexty), next_score));
                }
            }
        }
    }

    // part 1
    println!("{}", min);

    // part 2
    println!("{}", total);
}
