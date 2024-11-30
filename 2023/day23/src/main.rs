use std::{
    collections::{HashMap, HashSet, VecDeque},
    fs,
};

#[derive(Debug)]
struct State {
    steps: usize,
    pos: (i32, i32),
    seen: HashSet<(i32, i32)>,
    last_junction: (i32, i32),
}

fn dfs(pos: usize, end: usize, edges: &[Vec<(usize, usize)>], seen: &mut [bool]) -> usize {
    seen[pos] = true;
    let r = edges[pos]
        .iter()
        .map(|n| {
            if seen[n.0] {
                0
            } else if n.0 == end {
                n.1
            } else {
                let s = dfs(n.0, end, edges, seen);
                if s > 0 {
                    n.1 + s
                } else {
                    0
                }
            }
        })
        .max()
        .unwrap();
    seen[pos] = false;
    r
}

fn insert_edge(
    a: (i32, i32),
    b: (i32, i32),
    steps: usize,
    junctions: &mut HashMap<(i32, i32), usize>,
    edges: &mut [Vec<(usize, usize)>],
    w: usize,
    part1: bool,
) {
    let bil = junctions.len();
    let bi = *junctions.entry(b).or_insert(bil);
    let p1 = (bi, steps);
    if !edges[a.1 as usize * w + a.0 as usize].contains(&p1) {
        edges[a.1 as usize * w + a.0 as usize].push(p1);
    }
    if !part1 {
        let ail = junctions.len();
        let ai = *junctions.entry(a).or_insert(ail);
        // for part 2, edges must be bidirectional
        let p2 = (ai, steps);
        if !edges[b.1 as usize * w + b.0 as usize].contains(&p2) {
            edges[b.1 as usize * w + b.0 as usize].push(p2);
        }
    }
}

fn main() {
    for part1 in [true, false] {
        let input = fs::read_to_string("input.txt").expect("Could not read file");
        let lines = input.lines().collect::<Vec<_>>();
        let grid = lines
            .iter()
            .flat_map(|l| l.as_bytes())
            .copied()
            .collect::<Vec<_>>();
        let w = lines[0].len();
        let h = lines.len();

        // find start
        let mut start = (0i32, 0i32);
        for (x, c) in grid.iter().enumerate().take(w) {
            if *c == b'.' {
                start = (x as i32, 0i32);
                break;
            }
        }

        // find end
        let mut end = (0i32, 0i32);
        for (x, c) in grid.iter().skip((h - 1) * w).enumerate() {
            if *c == b'.' {
                end = (x as i32, h as i32 - 1);
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

        let mut junctions = HashMap::new();
        junctions.insert(start, 0);
        let mut edges = vec![Vec::<(usize, usize)>::new(); w * h];
        while let Some(mut s) = queue.pop_front() {
            // insert edge to the end
            if s.pos == end {
                insert_edge(
                    s.last_junction,
                    s.pos,
                    s.steps,
                    &mut junctions,
                    &mut edges,
                    w,
                    part1,
                );
                continue;
            }

            // check if we're at a junction
            let mut c = 0;
            for dir in [(0, 1), (0, -1), (1, 0), (-1, 0)] {
                let nx = s.pos.0 + dir.0;
                let ny = s.pos.1 + dir.1;
                if nx < 0 || ny < 0 || nx >= w as i32 || ny >= h as i32 {
                    continue;
                }
                if grid[ny as usize * w + nx as usize] == b'#' {
                    continue;
                }
                c += 1;
            }

            // if we're at a junction, ...
            if c > 2 {
                let have_seen = junctions.contains_key(&s.pos);

                // ... insert edge to the last junction
                insert_edge(
                    s.last_junction,
                    s.pos,
                    s.steps,
                    &mut junctions,
                    &mut edges,
                    w,
                    part1,
                );

                // if we haven't seen this junction before, make it the current
                // junction and reset steps
                if have_seen {
                    continue;
                }
                s.last_junction = s.pos;
                s.steps = 0;
            }

            // move to the next position(s)
            for dir in [(0, 1), (0, -1), (1, 0), (-1, 0)] {
                let nx = s.pos.0 + dir.0;
                let ny = s.pos.1 + dir.1;
                if nx < 0 || ny < 0 || nx >= w as i32 || ny >= h as i32 {
                    continue;
                }
                if grid[ny as usize * w + nx as usize] == b'#' {
                    continue;
                }
                if s.seen.contains(&(nx, ny)) {
                    continue;
                }
                if part1 {
                    if dir == (1, 0) && grid[ny as usize * w + nx as usize] == b'<' {
                        continue;
                    }
                    if dir == (-1, 0) && grid[ny as usize * w + nx as usize] == b'>' {
                        continue;
                    }
                    if dir == (0, 1) && grid[ny as usize * w + nx as usize] == b'^' {
                        continue;
                    }
                    if dir == (0, -1) && grid[ny as usize * w + nx as usize] == b'v' {
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

        // compress edges
        let mut le = vec![Vec::new(); junctions.len()];
        for (pos, &i) in &junctions {
            std::mem::swap(&mut le[i], &mut edges[pos.1 as usize * w + pos.0 as usize]);
        }

        // perform DFS to find the longest path
        let mut seen = vec![false; le.len()];
        let max = dfs(0, junctions[&end], &le, &mut seen);
        println!("{}", max);
    }
}
