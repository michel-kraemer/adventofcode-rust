use std::cmp::Reverse;
use std::collections::BinaryHeap;
use std::fs;

#[derive(PartialEq, Eq)]
struct State {
    score: usize,
    x: i32,
    y: i32,
    dx: i32,
    dy: i32,
    path: Vec<(i32, i32)>,
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

    // find start
    let mut start = (0, 0);
    'outer: for y in 0..height {
        for x in 0..width {
            if grid[y * width + x] == b'S' {
                start = (x as i32, y as i32);
                break 'outer;
            }
        }
    }

    // start facing east
    let mut queue = BinaryHeap::new();
    let start = State {
        score: 0,
        x: start.0,
        y: start.1,
        dx: 1,
        dy: 0,
        path: Vec::new(),
    };
    queue.push(Reverse(start));

    // a mapping of seen places and their lowest score
    let mut seen = vec![usize::MAX - 1000; width * height];

    // the minimum score it takes to get to the end
    let mut min = usize::MAX;

    // all best paths we have found
    let mut paths = Vec::new();

    while let Some(Reverse(State {
        score,
        x,
        y,
        dx: prev_dx,
        dy: prev_dy,
        path,
    })) = queue.pop()
    {
        if grid[y as usize * width + x as usize] == b'E' {
            if score > min {
                // This path is worse than any best path we've found before.
                // We can break here. No other path will be better.
                break;
            }
            paths.push(path.clone());
            min = score;
        }

        for (dx, dy) in [(1, 0), (-1, 0), (0, 1), (0, -1)] {
            let nx = x + dx;
            let ny = y + dy;

            let nscore = if (prev_dx == 0 && prev_dy == 0) || dx == prev_dx && dy == prev_dy {
                score + 1
            } else {
                score + 1001
            };

            let last_seen_score = seen[ny as usize * width + nx as usize];

            if nx >= 0
                && ny >= 0
                && nx < width as i32
                && ny < height as i32
                && grid[ny as usize * width + nx as usize] != b'#'
                // don't go back
                && !(prev_dx == 0 && prev_dy == -dy)
                && !(prev_dx == -dx && prev_dy == 0)
                // we might have stepped on a path where we were just about to turn
                // just continue and see how it goes
                && nscore <= last_seen_score + 1000
            {
                seen[ny as usize * width + nx as usize] = nscore;

                // SLOW!
                let mut new_path = path.clone();
                new_path.push((nx, ny));

                queue.push(Reverse(State {
                    score: nscore,
                    x: nx,
                    y: ny,
                    dx,
                    dy,
                    path: new_path,
                }));
            }
        }
    }

    let mut places_to_sit = vec![false; width * height];
    let mut total = 0;
    for p in &paths {
        for &(x, y) in p {
            if !places_to_sit[y as usize * width + x as usize] {
                places_to_sit[y as usize * width + x as usize] = true;
                total += 1;
            }
        }
    }

    // part 1
    println!("{}", min);

    // part 2
    println!("{}", total + 1);
}
