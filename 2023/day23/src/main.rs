use std::{
    collections::{HashSet, VecDeque},
    fs,
};

#[derive(Debug)]
struct State {
    steps: usize,
    pos: (i32, i32),
    seen: HashSet<(i32, i32)>,
    last_junction: (i32, i32),
}

type Grid<T> = Vec<Vec<T>>;

fn dfs(
    pos: (i32, i32),
    end: (i32, i32),
    edges: &Grid<Vec<(i32, i32, usize)>>,
    seen: &mut Grid<bool>,
) -> usize {
    seen[pos.1 as usize][pos.0 as usize] = true;
    let r = edges[pos.1 as usize][pos.0 as usize]
        .iter()
        .map(|n| {
            if seen[n.1 as usize][n.0 as usize] {
                0
            } else if (n.0, n.1) == end {
                n.2
            } else {
                let s = dfs((n.0, n.1), end, edges, seen);
                if s > 0 {
                    n.2 + s
                } else {
                    0
                }
            }
        })
        .max()
        .unwrap();
    seen[pos.1 as usize][pos.0 as usize] = false;
    r
}

fn insert_edge(
    a: (i32, i32),
    b: (i32, i32),
    steps: usize,
    edges: &mut Grid<Vec<(i32, i32, usize)>>,
    part1: bool,
) {
    let p1 = (b.0, b.1, steps);
    if !edges[a.1 as usize][a.0 as usize].contains(&p1) {
        edges[a.1 as usize][a.0 as usize].push(p1);
    }
    if !part1 {
        // for part 2, edges must be bidirectional
        let p2 = (a.0, a.1, steps);
        if !edges[b.1 as usize][b.0 as usize].contains(&p2) {
            edges[b.1 as usize][b.0 as usize].push(p2);
        }
    }
}

fn main() {
    for part1 in [true, false] {
        let input = fs::read_to_string("input.txt").expect("Could not read file");
        let grid = input
            .lines()
            .map(|l| l.chars().collect::<Vec<_>>())
            .collect::<Vec<_>>();

        // find start
        let mut start = (0i32, 0i32);
        for x in 0..grid[0].len() {
            if grid[0][x] == '.' {
                start = (x as i32, 0i32);
                break;
            }
        }

        // find end
        let mut end = (0i32, 0i32);
        for x in 0..grid[grid.len() - 1].len() {
            if grid[grid.len() - 1][x] == '.' {
                end = (x as i32, grid.len() as i32 - 1);
                break;
            }
        }

        // Find junctions and create edges between them. Each edge's value
        // is the number of steps between its junctions.
        let state = State {
            steps: 0,
            pos: start,
            seen: HashSet::new(),
            last_junction: start,
        };
        let mut queue = VecDeque::new();
        queue.push_back(state);

        let mut junctions = HashSet::new();
        junctions.insert(start);
        let mut edges = vec![vec![Vec::<(i32, i32, usize)>::new(); grid[0].len()]; grid.len()];
        while let Some(mut s) = queue.pop_front() {
            // insert edge to the end
            if s.pos == end {
                junctions.insert(s.pos);
                insert_edge(s.last_junction, s.pos, s.steps, &mut edges, part1);
                continue;
            }

            // check if we're at a junction
            let mut c = 0;
            for dir in [(0, 1), (0, -1), (1, 0), (-1, 0)] {
                let nx = s.pos.0 + dir.0;
                let ny = s.pos.1 + dir.1;
                if nx < 0 || ny < 0 || nx >= grid[0].len() as i32 || ny >= grid.len() as i32 {
                    continue;
                }
                if grid[ny as usize][nx as usize] == '#' {
                    continue;
                }
                c += 1;
            }

            // if we're at a junction, ...
            if c > 2 {
                // ... insert edge to the last junction
                insert_edge(s.last_junction, s.pos, s.steps, &mut edges, part1);

                // if we haven't seen this junction before, make it the current
                // junction and reset the steps
                if junctions.contains(&s.pos) {
                    continue;
                }
                junctions.insert(s.pos);
                s.last_junction = s.pos;
                s.steps = 0;
            }

            // move to the next position(s)
            for dir in [(0, 1), (0, -1), (1, 0), (-1, 0)] {
                let nx = s.pos.0 + dir.0;
                let ny = s.pos.1 + dir.1;
                if nx < 0 || ny < 0 || nx >= grid[0].len() as i32 || ny >= grid.len() as i32 {
                    continue;
                }
                if grid[ny as usize][nx as usize] == '#' {
                    continue;
                }
                if s.seen.contains(&(nx, ny)) {
                    continue;
                }
                if part1 {
                    if dir == (1, 0) && grid[ny as usize][nx as usize] == '<' {
                        continue;
                    }
                    if dir == (-1, 0) && grid[ny as usize][nx as usize] == '>' {
                        continue;
                    }
                    if dir == (0, 1) && grid[ny as usize][nx as usize] == '^' {
                        continue;
                    }
                    if dir == (0, -1) && grid[ny as usize][nx as usize] == 'v' {
                        continue;
                    }
                }
                let mut new_seen = s.seen.clone();
                new_seen.insert((nx, ny));
                let new_state = State {
                    steps: s.steps + 1,
                    pos: (nx, ny),
                    seen: new_seen,
                    last_junction: s.last_junction,
                };
                queue.push_back(new_state);
            }
        }

        // perform DFS to find the longest path
        let mut seen = vec![vec![false; grid[0].len()]; grid.len()];
        let max = dfs(start, end, &edges, &mut seen);
        println!("{}", max);
    }
}
